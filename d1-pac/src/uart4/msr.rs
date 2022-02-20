#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Line State of Data Carrier Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCD_A {
    #[doc = "0: `0`"]
    DEASSERTED = 0,
    #[doc = "1: `1`"]
    ASSERTED = 1,
}
impl From<DCD_A> for bool {
    #[inline(always)]
    fn from(variant: DCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dcd` reader - Line State of Data Carrier Detect"]
pub struct DCD_R(crate::FieldReader<bool, DCD_A>);
impl DCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCD_A {
        match self.bits {
            false => DCD_A::DEASSERTED,
            true => DCD_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        **self == DCD_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == DCD_A::ASSERTED
    }
}
impl core::ops::Deref for DCD_R {
    type Target = crate::FieldReader<bool, DCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Line State of Ring Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RI_A {
    #[doc = "0: `0`"]
    DEASSERTED = 0,
    #[doc = "1: `1`"]
    ASSERTED = 1,
}
impl From<RI_A> for bool {
    #[inline(always)]
    fn from(variant: RI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ri` reader - Line State of Ring Indicator"]
pub struct RI_R(crate::FieldReader<bool, RI_A>);
impl RI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RI_A {
        match self.bits {
            false => RI_A::DEASSERTED,
            true => RI_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        **self == RI_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == RI_A::ASSERTED
    }
}
impl core::ops::Deref for RI_R {
    type Target = crate::FieldReader<bool, RI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Line State of Data Set Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSR_A {
    #[doc = "0: `0`"]
    DEASSERTED = 0,
    #[doc = "1: `1`"]
    ASSERTED = 1,
}
impl From<DSR_A> for bool {
    #[inline(always)]
    fn from(variant: DSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dsr` reader - Line State of Data Set Ready"]
pub struct DSR_R(crate::FieldReader<bool, DSR_A>);
impl DSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSR_A {
        match self.bits {
            false => DSR_A::DEASSERTED,
            true => DSR_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        **self == DSR_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == DSR_A::ASSERTED
    }
}
impl core::ops::Deref for DSR_R {
    type Target = crate::FieldReader<bool, DSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Line State of Clear To Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_A {
    #[doc = "0: `0`"]
    DEASSERTED = 0,
    #[doc = "1: `1`"]
    ASSERTED = 1,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cts` reader - Line State of Clear To Send"]
pub struct CTS_R(crate::FieldReader<bool, CTS_A>);
impl CTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTS_A {
        match self.bits {
            false => CTS_A::DEASSERTED,
            true => CTS_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `DEASSERTED`"]
    #[inline(always)]
    pub fn is_deasserted(&self) -> bool {
        **self == CTS_A::DEASSERTED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == CTS_A::ASSERTED
    }
}
impl core::ops::Deref for CTS_R {
    type Target = crate::FieldReader<bool, CTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Delta Data Carrier Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDCD_A {
    #[doc = "0: `0`"]
    NO_CHANGE = 0,
    #[doc = "1: `1`"]
    CHANGE = 1,
}
impl From<DDCD_A> for bool {
    #[inline(always)]
    fn from(variant: DDCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ddcd` reader - Delta Data Carrier Detect"]
pub struct DDCD_R(crate::FieldReader<bool, DDCD_A>);
impl DDCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDCD_A {
        match self.bits {
            false => DDCD_A::NO_CHANGE,
            true => DDCD_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == DDCD_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        **self == DDCD_A::CHANGE
    }
}
impl core::ops::Deref for DDCD_R {
    type Target = crate::FieldReader<bool, DDCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Trailing Edge Ring Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERI_A {
    #[doc = "0: `0`"]
    NO_CHANGE = 0,
    #[doc = "1: `1`"]
    CHANGE = 1,
}
impl From<TERI_A> for bool {
    #[inline(always)]
    fn from(variant: TERI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `teri` reader - Trailing Edge Ring Indicator"]
pub struct TERI_R(crate::FieldReader<bool, TERI_A>);
impl TERI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TERI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERI_A {
        match self.bits {
            false => TERI_A::NO_CHANGE,
            true => TERI_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == TERI_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        **self == TERI_A::CHANGE
    }
}
impl core::ops::Deref for TERI_R {
    type Target = crate::FieldReader<bool, TERI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Delta Data Set Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDSR_A {
    #[doc = "0: `0`"]
    NO_CHANGE = 0,
    #[doc = "1: `1`"]
    CHANGE = 1,
}
impl From<DDSR_A> for bool {
    #[inline(always)]
    fn from(variant: DDSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ddsr` reader - Delta Data Set Ready"]
pub struct DDSR_R(crate::FieldReader<bool, DDSR_A>);
impl DDSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDSR_A {
        match self.bits {
            false => DDSR_A::NO_CHANGE,
            true => DDSR_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == DDSR_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        **self == DDSR_A::CHANGE
    }
}
impl core::ops::Deref for DDSR_R {
    type Target = crate::FieldReader<bool, DDSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Delta Clear to Send\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTS_A {
    #[doc = "0: `0`"]
    NO_CHANGE = 0,
    #[doc = "1: `1`"]
    CHANGE = 1,
}
impl From<DCTS_A> for bool {
    #[inline(always)]
    fn from(variant: DCTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dcts` reader - Delta Clear to Send"]
pub struct DCTS_R(crate::FieldReader<bool, DCTS_A>);
impl DCTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCTS_A {
        match self.bits {
            false => DCTS_A::NO_CHANGE,
            true => DCTS_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == DCTS_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        **self == DCTS_A::CHANGE
    }
}
impl core::ops::Deref for DCTS_R {
    type Target = crate::FieldReader<bool, DCTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Line State of Data Carrier Detect"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Line State of Ring Indicator"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Line State of Data Set Ready"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Line State of Clear To Send"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Delta Data Carrier Detect"]
    #[inline(always)]
    pub fn ddcd(&self) -> DDCD_R {
        DDCD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge Ring Indicator"]
    #[inline(always)]
    pub fn teri(&self) -> TERI_R {
        TERI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Delta Data Set Ready"]
    #[inline(always)]
    pub fn ddsr(&self) -> DDSR_R {
        DDSR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Delta Clear to Send"]
    #[inline(always)]
    pub fn dcts(&self) -> DCTS_R {
        DCTS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "UART Modem Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
