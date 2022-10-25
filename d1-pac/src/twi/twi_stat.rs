#[doc = "Register `twi_stat` reader"]
pub struct R(crate::R<TWI_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sta` reader - "]
pub type STA_R = crate::FieldReader<u8, STA_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STA_A {
    #[doc = "0: Bus error"]
    BE = 0,
    #[doc = "8: START condition transmitted"]
    SCT = 8,
    #[doc = "16: Repeated START condition transmitted"]
    RSCT = 16,
    #[doc = "24: Address + Write bit transmitted, ACK received"]
    AWBT_AR = 24,
    #[doc = "32: Address + Write bit transmitted, ACK not received"]
    AWBT_ANR = 32,
    #[doc = "40: Data byte transmitted in master mode, ACK received"]
    DBTM_AR = 40,
    #[doc = "48: Data byte transmitted in master mode, ACK not received"]
    DBTM_ANR = 48,
    #[doc = "56: Arbitration lost in address or data byte"]
    AL_A_DB = 56,
    #[doc = "64: Address + Read bit transmitted, ACK received"]
    ARBT_AR = 64,
    #[doc = "72: Address + Read bit transmitted, ACK not received"]
    ARBT_ANR = 72,
    #[doc = "80: Data byte received in master mode, ACK transmitted"]
    DBRM_AT = 80,
    #[doc = "88: Data byte received in master mode, not ACK transmitted"]
    DBRM_ANT = 88,
    #[doc = "96: Slave address + Write bit received, ACK transmitted"]
    SAWR_AT = 96,
    #[doc = "104: Arbitration lost in the address as master, slave address + Write bit received, ACK transmitted"]
    AL_AM_SAWR_AT = 104,
    #[doc = "112: General Call address received, ACK transmitted"]
    GCAR_AT = 112,
    #[doc = "120: Arbitration lost in the address as master, General Call address received, ACK transmitted"]
    AL_AM_GCAR_AT = 120,
    #[doc = "128: Data byte received after slave address received, ACK transmitted"]
    DBR_SAR_AT = 128,
    #[doc = "136: Data byte received after slave address received, not ACK transmitted"]
    DBR_SAR_ANT = 136,
    #[doc = "144: Data byte received after General Call received, ACK transmitted"]
    DBR_GCR_AT = 144,
    #[doc = "152: Data byte received after General Call received, not ACK transmitted"]
    DBR_GCR_ANT = 152,
    #[doc = "160: STOP or repeated START condition received in slave mode"]
    SRSCRS = 160,
    #[doc = "168: Slave address + Read bit received, ACK transmitted"]
    SARR_AT = 168,
    #[doc = "176: Arbitration lost in the address as master, slave address + Read bit received, ACK transmitted"]
    AL_AM_SARR_AT = 176,
    #[doc = "184: Data byte transmitted in slave mode, ACK received"]
    DBTS_AR = 184,
    #[doc = "192: Data byte transmitted in slave mode, ACK not received"]
    DBTS_ANR = 192,
    #[doc = "200: The Last byte transmitted in slave mode, ACK received"]
    LBTS_AR = 200,
    #[doc = "208: Second Address byte + Write bit transmitted, ACK received"]
    SAWT_AR = 208,
    #[doc = "216: Second Address byte + Write bit transmitted, ACK not received"]
    SAWT_ANR = 216,
    #[doc = "248: No relevant status information, INT_FLAG=0"]
    NRSI = 248,
}
impl From<STA_A> for u8 {
    #[inline(always)]
    fn from(variant: STA_A) -> Self {
        variant as _
    }
}
impl STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STA_A> {
        match self.bits {
            0 => Some(STA_A::BE),
            8 => Some(STA_A::SCT),
            16 => Some(STA_A::RSCT),
            24 => Some(STA_A::AWBT_AR),
            32 => Some(STA_A::AWBT_ANR),
            40 => Some(STA_A::DBTM_AR),
            48 => Some(STA_A::DBTM_ANR),
            56 => Some(STA_A::AL_A_DB),
            64 => Some(STA_A::ARBT_AR),
            72 => Some(STA_A::ARBT_ANR),
            80 => Some(STA_A::DBRM_AT),
            88 => Some(STA_A::DBRM_ANT),
            96 => Some(STA_A::SAWR_AT),
            104 => Some(STA_A::AL_AM_SAWR_AT),
            112 => Some(STA_A::GCAR_AT),
            120 => Some(STA_A::AL_AM_GCAR_AT),
            128 => Some(STA_A::DBR_SAR_AT),
            136 => Some(STA_A::DBR_SAR_ANT),
            144 => Some(STA_A::DBR_GCR_AT),
            152 => Some(STA_A::DBR_GCR_ANT),
            160 => Some(STA_A::SRSCRS),
            168 => Some(STA_A::SARR_AT),
            176 => Some(STA_A::AL_AM_SARR_AT),
            184 => Some(STA_A::DBTS_AR),
            192 => Some(STA_A::DBTS_ANR),
            200 => Some(STA_A::LBTS_AR),
            208 => Some(STA_A::SAWT_AR),
            216 => Some(STA_A::SAWT_ANR),
            248 => Some(STA_A::NRSI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BE`"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == STA_A::BE
    }
    #[doc = "Checks if the value of the field is `SCT`"]
    #[inline(always)]
    pub fn is_sct(&self) -> bool {
        *self == STA_A::SCT
    }
    #[doc = "Checks if the value of the field is `RSCT`"]
    #[inline(always)]
    pub fn is_rsct(&self) -> bool {
        *self == STA_A::RSCT
    }
    #[doc = "Checks if the value of the field is `AWBT_AR`"]
    #[inline(always)]
    pub fn is_awbt_ar(&self) -> bool {
        *self == STA_A::AWBT_AR
    }
    #[doc = "Checks if the value of the field is `AWBT_ANR`"]
    #[inline(always)]
    pub fn is_awbt_anr(&self) -> bool {
        *self == STA_A::AWBT_ANR
    }
    #[doc = "Checks if the value of the field is `DBTM_AR`"]
    #[inline(always)]
    pub fn is_dbtm_ar(&self) -> bool {
        *self == STA_A::DBTM_AR
    }
    #[doc = "Checks if the value of the field is `DBTM_ANR`"]
    #[inline(always)]
    pub fn is_dbtm_anr(&self) -> bool {
        *self == STA_A::DBTM_ANR
    }
    #[doc = "Checks if the value of the field is `AL_A_DB`"]
    #[inline(always)]
    pub fn is_al_a_db(&self) -> bool {
        *self == STA_A::AL_A_DB
    }
    #[doc = "Checks if the value of the field is `ARBT_AR`"]
    #[inline(always)]
    pub fn is_arbt_ar(&self) -> bool {
        *self == STA_A::ARBT_AR
    }
    #[doc = "Checks if the value of the field is `ARBT_ANR`"]
    #[inline(always)]
    pub fn is_arbt_anr(&self) -> bool {
        *self == STA_A::ARBT_ANR
    }
    #[doc = "Checks if the value of the field is `DBRM_AT`"]
    #[inline(always)]
    pub fn is_dbrm_at(&self) -> bool {
        *self == STA_A::DBRM_AT
    }
    #[doc = "Checks if the value of the field is `DBRM_ANT`"]
    #[inline(always)]
    pub fn is_dbrm_ant(&self) -> bool {
        *self == STA_A::DBRM_ANT
    }
    #[doc = "Checks if the value of the field is `SAWR_AT`"]
    #[inline(always)]
    pub fn is_sawr_at(&self) -> bool {
        *self == STA_A::SAWR_AT
    }
    #[doc = "Checks if the value of the field is `AL_AM_SAWR_AT`"]
    #[inline(always)]
    pub fn is_al_am_sawr_at(&self) -> bool {
        *self == STA_A::AL_AM_SAWR_AT
    }
    #[doc = "Checks if the value of the field is `GCAR_AT`"]
    #[inline(always)]
    pub fn is_gcar_at(&self) -> bool {
        *self == STA_A::GCAR_AT
    }
    #[doc = "Checks if the value of the field is `AL_AM_GCAR_AT`"]
    #[inline(always)]
    pub fn is_al_am_gcar_at(&self) -> bool {
        *self == STA_A::AL_AM_GCAR_AT
    }
    #[doc = "Checks if the value of the field is `DBR_SAR_AT`"]
    #[inline(always)]
    pub fn is_dbr_sar_at(&self) -> bool {
        *self == STA_A::DBR_SAR_AT
    }
    #[doc = "Checks if the value of the field is `DBR_SAR_ANT`"]
    #[inline(always)]
    pub fn is_dbr_sar_ant(&self) -> bool {
        *self == STA_A::DBR_SAR_ANT
    }
    #[doc = "Checks if the value of the field is `DBR_GCR_AT`"]
    #[inline(always)]
    pub fn is_dbr_gcr_at(&self) -> bool {
        *self == STA_A::DBR_GCR_AT
    }
    #[doc = "Checks if the value of the field is `DBR_GCR_ANT`"]
    #[inline(always)]
    pub fn is_dbr_gcr_ant(&self) -> bool {
        *self == STA_A::DBR_GCR_ANT
    }
    #[doc = "Checks if the value of the field is `SRSCRS`"]
    #[inline(always)]
    pub fn is_srscrs(&self) -> bool {
        *self == STA_A::SRSCRS
    }
    #[doc = "Checks if the value of the field is `SARR_AT`"]
    #[inline(always)]
    pub fn is_sarr_at(&self) -> bool {
        *self == STA_A::SARR_AT
    }
    #[doc = "Checks if the value of the field is `AL_AM_SARR_AT`"]
    #[inline(always)]
    pub fn is_al_am_sarr_at(&self) -> bool {
        *self == STA_A::AL_AM_SARR_AT
    }
    #[doc = "Checks if the value of the field is `DBTS_AR`"]
    #[inline(always)]
    pub fn is_dbts_ar(&self) -> bool {
        *self == STA_A::DBTS_AR
    }
    #[doc = "Checks if the value of the field is `DBTS_ANR`"]
    #[inline(always)]
    pub fn is_dbts_anr(&self) -> bool {
        *self == STA_A::DBTS_ANR
    }
    #[doc = "Checks if the value of the field is `LBTS_AR`"]
    #[inline(always)]
    pub fn is_lbts_ar(&self) -> bool {
        *self == STA_A::LBTS_AR
    }
    #[doc = "Checks if the value of the field is `SAWT_AR`"]
    #[inline(always)]
    pub fn is_sawt_ar(&self) -> bool {
        *self == STA_A::SAWT_AR
    }
    #[doc = "Checks if the value of the field is `SAWT_ANR`"]
    #[inline(always)]
    pub fn is_sawt_anr(&self) -> bool {
        *self == STA_A::SAWT_ANR
    }
    #[doc = "Checks if the value of the field is `NRSI`"]
    #[inline(always)]
    pub fn is_nrsi(&self) -> bool {
        *self == STA_A::NRSI
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TWI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_stat](index.html) module"]
pub struct TWI_STAT_SPEC;
impl crate::RegisterSpec for TWI_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_stat::R](R) reader structure"]
impl crate::Readable for TWI_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets twi_stat to value 0"]
impl crate::Resettable for TWI_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
