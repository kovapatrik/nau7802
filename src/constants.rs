pub(super) const NAU7802_ADDRESS: u8 = 0x2A;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
  I2CError(embedded_hal::i2c::ErrorKind),
  Timeout,
  NotReady,
  InvalidData,
  CalibrationError,
  InvalidRevisionId(u8),
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Ldo {
  L4v5,
  L4v2,
  L3v9,
  L3v6,
  L3v3,
  L3v0,
  L2v7,
  L2v4,
  External,
}

impl TryFrom<u8> for Ldo {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0b000 => Ok(Ldo::L4v5),
      0b001 => Ok(Ldo::L4v2),
      0b010 => Ok(Ldo::L3v9),
      0b011 => Ok(Ldo::L3v6),
      0b100 => Ok(Ldo::L3v3),
      0b101 => Ok(Ldo::L3v0),
      0b110 => Ok(Ldo::L2v7),
      0b111 => Ok(Ldo::L2v4),
      _ => Err(()),
    }
  }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Gain {
  G1,
  G2,
  G4,
  G8,
  G16,
  G32,
  G64,
  G128,
}

impl TryFrom<u8> for Gain {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0b000 => Ok(Gain::G1),
      0b001 => Ok(Gain::G2),
      0b010 => Ok(Gain::G4),
      0b011 => Ok(Gain::G8),
      0b100 => Ok(Gain::G16),
      0b101 => Ok(Gain::G32),
      0b110 => Ok(Gain::G64),
      0b111 => Ok(Gain::G128),
      _ => Err(()),
    }
  }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SampleRate {
  SPS10 = 0b000,
  SPS20 = 0b001,
  SPS40 = 0b010,
  SPS80 = 0b011,
  SPS320 = 0b111,
}

impl TryFrom<u8> for SampleRate {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0b000 => Ok(SampleRate::SPS10),
      0b001 => Ok(SampleRate::SPS20),
      0b010 => Ok(SampleRate::SPS40),
      0b011 => Ok(SampleRate::SPS80),
      0b111 => Ok(SampleRate::SPS320),
      _ => Err(()),
    }
  }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum CalibrationMode {
  Internal = 0,
  Offset = 2,
  Gain = 3,
}
