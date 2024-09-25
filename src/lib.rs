#![no_std]

mod constants;
mod registers;

use constants::NAU7802_ADDRESS;
use embedded_hal::i2c::Error as _;
use registers::Register;

pub use constants::{CalibrationMode, Error, Gain, Ldo, SampleRate};

pub struct Nau7802<I2C, Delay> {
  i2c: I2C,
  address: u8,
  delay: Delay,
}

impl<I2C, Delay> Nau7802<I2C, Delay>
where
  I2C: embedded_hal::i2c::I2c,
  Delay: embedded_hal::delay::DelayNs,
{
  pub fn new(i2c: I2C, delay: Delay) -> Result<Self, Error> {
    Self::new_with_options(i2c, delay, Ldo::L3v3, Gain::G128, SampleRate::SPS80)
  }

  pub fn new_with_options(
    i2c: I2C,
    delay: Delay,
    ldo: Ldo,
    gain: Gain,
    sample_rate: SampleRate,
  ) -> Result<Self, Error> {
    let mut device = Nau7802 {
      i2c,
      address: NAU7802_ADDRESS,
      delay,
    };

    device.reset().unwrap();
    device.enable(true).unwrap();

    if device.read_revision_id().unwrap() != 0xF {
      return Err(Error::InvalidRevisionId(device.read_revision_id().unwrap()));
    }

    device.set_ldo(ldo).unwrap();
    device.set_gain(gain).unwrap();
    device.set_sample_rate(sample_rate).unwrap();

    // Disable ADC chopper clock
    let adc = device.read_register(Register::AdcOtpB2)?;
    let mut adc = registers::AdcRegister(adc);
    adc.set_reg_chps(0x3);
    device.write_register(Register::AdcOtpB2, adc.0)?;

    // Use low ESR capacitor
    let pga = device.read_register(Register::Pga)?;
    let mut pga = registers::Pga(pga);
    pga.set_ldomode(false);
    device.write_register(Register::Pga, pga.0)?;

    // PGA stabilizer capacitor enable on output
    let power_ctrl = device.read_register(Register::PowerCtrl)?;
    let mut power_ctrl = registers::PowerCtrl(power_ctrl);
    power_ctrl.set_pga_cap_en(true);
    device.write_register(Register::PowerCtrl, power_ctrl.0)?;

    Ok(device)
  }

  pub fn enable(&mut self, is_power_up: bool) -> Result<(), Error> {
    let pu_ctrl = self.read_register(Register::PuCtrl)?;
    let mut pu_ctrl = registers::PuCtrl(pu_ctrl);

    if !is_power_up {
      pu_ctrl.set_pua(false);
      pu_ctrl.set_pud(false);
      self.write_register(Register::PuCtrl, pu_ctrl.0)?;
    } else {
      pu_ctrl.set_pud(true);
      pu_ctrl.set_pua(true);
      self.write_register(Register::PuCtrl, pu_ctrl.0)?;
      self.delay.delay_ms(600);
      pu_ctrl.set_cs(true);
      self.write_register(Register::PuCtrl, pu_ctrl.0)?;

      let pu_ctrl = self.read_register(Register::PuCtrl)?;
      let pu_ctrl = registers::PuCtrl(pu_ctrl);

      if !pu_ctrl.pur() {
        return Err(Error::NotReady);
      }
    }

    Ok(())
  }

  pub fn reset(&mut self) -> Result<(), Error> {
    let pu_ctrl = self.read_register(Register::PuCtrl)?;
    let mut pu_ctrl = registers::PuCtrl(pu_ctrl);

    pu_ctrl.set_rr(true);
    self.write_register(Register::PuCtrl, pu_ctrl.0)?;
    self.delay.delay_ms(10);

    pu_ctrl.set_rr(false);
    pu_ctrl.set_pud(true);
    self.write_register(Register::PuCtrl, pu_ctrl.0)?;
    self.delay.delay_ms(1);

    let pu_ctrl = self.read_register(Register::PuCtrl)?;
    let pu_ctrl = registers::PuCtrl(pu_ctrl);

    if !pu_ctrl.pur() {
      return Err(Error::NotReady);
    }
    Ok(())
  }

  pub fn available(&mut self) -> Result<bool, Error> {
    let pu_ctrl = self.read_register(Register::PuCtrl)?;
    let pu_ctrl = registers::PuCtrl(pu_ctrl);
    Ok(pu_ctrl.cr())
  }

  pub fn read(&mut self) -> Result<i32, Error> {
    let mut data = [0; 3];
    self
      .i2c
      .write_read(self.address, &[Register::AdcOB2.address()], &mut data)
      .map_err(|e| Error::I2CError(e.kind()))?;

    let mut value = ((data[0] as u32) << 16) | ((data[1] as u32) << 8) | (data[2] as u32);
    if value & 0x800000 != 0 {
      value |= 0xFF000000;
    }
    Ok(value as i32)
  }

  pub fn set_ldo(&mut self, ldo: Ldo) -> Result<(), Error> {
    let pu_ctrl = self.read_register(Register::PuCtrl)?;
    let mut pu_ctrl = registers::PuCtrl(pu_ctrl);
    match ldo {
      Ldo::External => {
        pu_ctrl.set_avdds(false);
        self.write_register(Register::PuCtrl, pu_ctrl.0)?;
      }
      _ => {
        pu_ctrl.set_avdds(true);
        self.write_register(Register::PuCtrl, pu_ctrl.0)?;
        let ctrl1 = self.read_register(Register::Ctrl1)?;
        let mut ctrl1 = registers::Ctrl1(ctrl1);
        ctrl1.set_vldo(ldo as u8);
        self.write_register(Register::Ctrl1, ctrl1.0)?;
      }
    }
    Ok(())
  }

  pub fn get_ldo(&mut self) -> Result<Ldo, Error> {
    let pu_ctrl = self.read_register(Register::PuCtrl)?;
    let pu_ctrl = registers::PuCtrl(pu_ctrl);

    if !pu_ctrl.avdds() {
      return Ok(Ldo::External);
    }

    let ctrl1 = self.read_register(Register::Ctrl1)?;
    let ctrl1 = registers::Ctrl1(ctrl1);
    match Ldo::try_from(ctrl1.vldo()) {
      Ok(ldo) => Ok(ldo),
      Err(_) => Err(Error::InvalidData),
    }
  }

  pub fn set_gain(&mut self, gain: Gain) -> Result<(), Error> {
    let ctrl1 = self.read_register(Register::Ctrl1)?;
    let mut ctrl1 = registers::Ctrl1(ctrl1);
    ctrl1.set_gains(gain as u8);
    self.write_register(Register::Ctrl1, ctrl1.0)?;
    Ok(())
  }

  pub fn get_gain(&mut self) -> Result<Gain, Error> {
    let ctrl1 = self.read_register(Register::Ctrl1)?;
    let ctrl1 = registers::Ctrl1(ctrl1);
    match Gain::try_from(ctrl1.gains()) {
      Ok(gain) => Ok(gain),
      Err(_) => Err(Error::InvalidData),
    }
  }

  pub fn set_sample_rate(&mut self, sample_rate: SampleRate) -> Result<(), Error> {
    let ctrl2 = self.read_register(Register::Ctrl2)?;
    let mut ctrl2 = registers::Ctrl2(ctrl2);
    ctrl2.set_crs(sample_rate as u8);
    self.write_register(Register::Ctrl2, ctrl2.0)?;
    Ok(())
  }

  pub fn get_sample_rate(&mut self) -> Result<SampleRate, Error> {
    let ctrl2 = self.read_register(Register::Ctrl2)?;
    let ctrl2 = registers::Ctrl2(ctrl2);
    match SampleRate::try_from(ctrl2.crs()) {
      Ok(sample_rate) => Ok(sample_rate),
      Err(_) => Err(Error::InvalidData),
    }
  }

  pub fn calibrate(&mut self, calibration_mode: CalibrationMode) -> Result<(), Error> {
    let ctrl2 = self.read_register(Register::Ctrl2)?;
    let mut ctrl2 = registers::Ctrl2(ctrl2);

    ctrl2.set_calmod(calibration_mode as u8);
    ctrl2.set_cals(true);
    self.write_register(Register::Ctrl2, ctrl2.0)?;

    loop {
      let ctrl2 = self.read_register(Register::Ctrl2)?;
      let ctrl2 = registers::Ctrl2(ctrl2);
      if !ctrl2.cals() {
        break;
      }
      self.delay.delay_ms(10);
    }

    if ctrl2.cal_err() {
      return Err(Error::CalibrationError);
    }
    Ok(())
  }

  fn read_revision_id(&mut self) -> Result<u8, Error> {
    let device_rev = self.read_register(Register::DeviceRev)?;
    let device_rev = registers::DeviceRev(device_rev);
    Ok(device_rev.revision_id())
  }

  fn read_register(&mut self, register: Register) -> Result<u8, Error> {
    let mut data = [0];
    self
      .i2c
      .write_read(self.address, &[register.address()], &mut data)
      .map_err(|e| Error::I2CError(e.kind()))?;
    Ok(u8::from_le_bytes(data))
  }

  fn write_register(&mut self, register: Register, value: u8) -> Result<(), Error> {
    self
      .i2c
      .write(self.address, &[register.address(), value])
      .map_err(|e| Error::I2CError(e.kind()))
  }
}
