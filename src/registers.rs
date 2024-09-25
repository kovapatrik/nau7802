use bitfield::bitfield;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Register {
  PuCtrl = 0x00,
  Ctrl1,
  Ctrl2,
  OCal1B2,
  OCal1B1,
  OCal1B0,
  GCal1B3,
  GCal1B2,
  GCal1B1,
  GCal1B0,
  OCal2B2,
  OCal2B1,
  OCal2B0,
  GCal2B3,
  GCal2B2,
  GCal2B1,
  GCal2B0,
  I2CControl,
  AdcOB2,
  AdcOB1,
  AdcOB0,
  AdcOtpB2, // Shared register
  OtpB1,
  OtpB0,
  Pga = 0x1B,
  PowerCtrl = 0x1C,
  DeviceRev = 0x1F,
}

impl Register {
  pub fn address(&self) -> u8 {
    *self as u8
  }
}

bitfield! {
  pub struct PuCtrl(u8);
  impl Debug;
  pub avdds, set_avdds: 7;
  pub oscs, set_oscs: 6;
  pub cr, _: 5;
  pub cs, set_cs: 4;
  pub pur, _: 3;
  pub pua, set_pua: 2;
  pub pud, set_pud: 1;
  pub rr, set_rr: 0;
}

bitfield! {
  pub struct Ctrl1(u8);
  impl Debug;
  pub crp, set_crp: 7;
  pub drdy_sel, set_drdy_sel: 6;
  pub vldo, set_vldo: 5, 3;
  pub gains, set_gains: 2, 0;
}

bitfield! {
  pub struct Ctrl2(u8);
  impl Debug;
  pub chs, set_chs: 7;
  pub crs, set_crs: 6, 4;
  pub cal_err, _: 3;
  pub cals, set_cals: 2;
  pub calmod, set_calmod: 1, 0;
}

bitfield! {
  pub struct I2CControl(u8);
  impl Debug;
  pub crsd, set_crsd: 7;
  pub fdr, set_fdr: 6;
  pub spe, set_spe: 5;
  pub wpd, set_wpd: 4;
  pub si, set_si: 3;
  pub bopga, set_bopga: 2;
  pub ts, set_ts: 1;
  pub bgpcp, set_bgpcp: 0;
}

bitfield! {
  pub struct AdcRegister(u8);
  impl Debug;
  pub reg_chps1, set_reg_chps1: 5;
  pub reg_chps0, set_reg_chps0: 4;
  pub adc_vcm1, set_adc_vcm1: 3;
  pub adc_vcm0, set_adc_vcm0: 2;
  pub reg_chp, set_reg_chp: 1, 0;
}

bitfield! {
  pub struct Pga(u8);
  impl Debug;
  pub rd_otp_sel, set_rd_otp_sel: 7;
  pub ldomode, set_ldomode: 6;
  pub pga_output_buffer_enable, set_pga_output_buffer_enable: 5;
  pub pga_bypass_enable, set_pga_bypass_enable: 4;
  pub pgainv, set_pgainv: 3;
  pub pgachpdis, set_pgachpdis: 0;
}

bitfield! {
  pub struct PowerCtrl(u8);
  impl Debug;
  pub pga_cap_en, set_pga_cap_en: 7;
  pub master_bias_current, set_master_bias_current: 6, 4;
  pub adc_current, set_adc_current: 3, 2;
  pub pga_current, set_pga_current: 1, 0;
}

bitfield! {
  pub struct DeviceRev(u8);
  impl Debug;
  pub revision_id, _: 3, 0;
}