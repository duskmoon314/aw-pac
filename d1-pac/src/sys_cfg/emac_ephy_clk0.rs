#[doc = "Register `EMAC_EPHY_CLK0` reader"]
pub struct R(crate::R<EMAC_EPHY_CLK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_EPHY_CLK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_EPHY_CLK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_EPHY_CLK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_EPHY_CLK0` writer"]
pub struct W(crate::W<EMAC_EPHY_CLK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_EPHY_CLK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EMAC_EPHY_CLK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_EPHY_CLK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BPS_EFUSE` reader - "]
pub struct BPS_EFUSE_R(crate::FieldReader<u8>);
impl BPS_EFUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BPS_EFUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BPS_EFUSE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BPS_EFUSE` writer - "]
pub struct BPS_EFUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> BPS_EFUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XMII_SEL_A {
    #[doc = "0: `0`"]
    INTERNAL = 0,
    #[doc = "1: `1`"]
    EXTERNAL = 1,
}
impl From<XMII_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: XMII_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XMII_SEL` reader - "]
pub struct XMII_SEL_R(crate::FieldReader<bool>);
impl XMII_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XMII_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XMII_SEL_A {
        match self.bits {
            false => XMII_SEL_A::INTERNAL,
            true => XMII_SEL_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == XMII_SEL_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == XMII_SEL_A::EXTERNAL
    }
}
impl core::ops::Deref for XMII_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XMII_SEL` writer - "]
pub struct XMII_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XMII_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XMII_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(XMII_SEL_A::INTERNAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(XMII_SEL_A::EXTERNAL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPHY_MODE_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    SIMULATION = 1,
    #[doc = "2: `10`"]
    AFE_TEST = 2,
}
impl From<EPHY_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPHY_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPHY_MODE` reader - "]
pub struct EPHY_MODE_R(crate::FieldReader<u8>);
impl EPHY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPHY_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPHY_MODE_A> {
        match self.bits {
            0 => Some(EPHY_MODE_A::NORMAL),
            1 => Some(EPHY_MODE_A::SIMULATION),
            2 => Some(EPHY_MODE_A::AFE_TEST),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == EPHY_MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SIMULATION`"]
    #[inline(always)]
    pub fn is_simulation(&self) -> bool {
        **self == EPHY_MODE_A::SIMULATION
    }
    #[doc = "Checks if the value of the field is `AFE_TEST`"]
    #[inline(always)]
    pub fn is_afe_test(&self) -> bool {
        **self == EPHY_MODE_A::AFE_TEST
    }
}
impl core::ops::Deref for EPHY_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPHY_MODE` writer - "]
pub struct EPHY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPHY_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPHY_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(EPHY_MODE_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn simulation(self) -> &'a mut W {
        self.variant(EPHY_MODE_A::SIMULATION)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn afe_test(self) -> &'a mut W {
        self.variant(EPHY_MODE_A::AFE_TEST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 25)) | ((value as u32 & 3) << 25);
        self.w
    }
}
#[doc = "Field `PHY_ADDR` reader - "]
pub struct PHY_ADDR_R(crate::FieldReader<u8>);
impl PHY_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PHY_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_ADDR_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_ADDR` writer - "]
pub struct PHY_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SEL_A {
    #[doc = "0: `0`"]
    F25M = 0,
    #[doc = "1: `1`"]
    F24M = 1,
}
impl From<CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SEL` reader - "]
pub struct CLK_SEL_R(crate::FieldReader<bool>);
impl CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SEL_A {
        match self.bits {
            false => CLK_SEL_A::F25M,
            true => CLK_SEL_A::F24M,
        }
    }
    #[doc = "Checks if the value of the field is `F25M`"]
    #[inline(always)]
    pub fn is_f25m(&self) -> bool {
        **self == CLK_SEL_A::F25M
    }
    #[doc = "Checks if the value of the field is `F24M`"]
    #[inline(always)]
    pub fn is_f24m(&self) -> bool {
        **self == CLK_SEL_A::F24M
    }
}
impl core::ops::Deref for CLK_SEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_SEL` writer - "]
pub struct CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn f25m(self) -> &'a mut W {
        self.variant(CLK_SEL_A::F25M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn f24m(self) -> &'a mut W {
        self.variant(CLK_SEL_A::F24M)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LED_POL_A {
    #[doc = "0: `0`"]
    HIGH = 0,
    #[doc = "1: `1`"]
    LOW = 1,
}
impl From<LED_POL_A> for bool {
    #[inline(always)]
    fn from(variant: LED_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LED_POL` reader - "]
pub struct LED_POL_R(crate::FieldReader<bool>);
impl LED_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LED_POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LED_POL_A {
        match self.bits {
            false => LED_POL_A::HIGH,
            true => LED_POL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == LED_POL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == LED_POL_A::LOW
    }
}
impl core::ops::Deref for LED_POL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LED_POL` writer - "]
pub struct LED_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LED_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LED_POL_A::HIGH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LED_POL_A::LOW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHUTDOWN_A {
    #[doc = "0: `0`"]
    POWER_UP = 0,
    #[doc = "1: `1`"]
    SHUT_DOWN = 1,
}
impl From<SHUTDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: SHUTDOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHUTDOWN` reader - "]
pub struct SHUTDOWN_R(crate::FieldReader<bool>);
impl SHUTDOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHUTDOWN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHUTDOWN_A {
        match self.bits {
            false => SHUTDOWN_A::POWER_UP,
            true => SHUTDOWN_A::SHUT_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        **self == SHUTDOWN_A::POWER_UP
    }
    #[doc = "Checks if the value of the field is `SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_shut_down(&self) -> bool {
        **self == SHUTDOWN_A::SHUT_DOWN
    }
}
impl core::ops::Deref for SHUTDOWN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHUTDOWN` writer - "]
pub struct SHUTDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUTDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHUTDOWN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut W {
        self.variant(SHUTDOWN_A::POWER_UP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn shut_down(self) -> &'a mut W {
        self.variant(SHUTDOWN_A::SHUT_DOWN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_SELECT_A {
    #[doc = "0: `0`"]
    EXTERNAL = 0,
    #[doc = "1: `1`"]
    INTERNAL = 1,
}
impl From<PHY_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHY_SELECT` reader - "]
pub struct PHY_SELECT_R(crate::FieldReader<bool>);
impl PHY_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHY_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHY_SELECT_A {
        match self.bits {
            false => PHY_SELECT_A::EXTERNAL,
            true => PHY_SELECT_A::INTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == PHY_SELECT_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == PHY_SELECT_A::INTERNAL
    }
}
impl core::ops::Deref for PHY_SELECT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_SELECT` writer - "]
pub struct PHY_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHY_SELECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(PHY_SELECT_A::EXTERNAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(PHY_SELECT_A::INTERNAL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMII_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RMII_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RMII_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMII_EN` reader - "]
pub struct RMII_EN_R(crate::FieldReader<bool>);
impl RMII_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMII_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMII_EN_A {
        match self.bits {
            false => RMII_EN_A::DISABLE,
            true => RMII_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RMII_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RMII_EN_A::ENABLE
    }
}
impl core::ops::Deref for RMII_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMII_EN` writer - "]
pub struct RMII_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMII_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RMII_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RMII_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `ETXDC` reader - "]
pub struct ETXDC_R(crate::FieldReader<u8>);
impl ETXDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ETXDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETXDC_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETXDC` writer - "]
pub struct ETXDC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETXDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 10)) | ((value as u32 & 7) << 10);
        self.w
    }
}
#[doc = "Field `ERXDC` reader - "]
pub struct ERXDC_R(crate::FieldReader<u8>);
impl ERXDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERXDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERXDC_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERXDC` writer - "]
pub struct ERXDC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERXDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERXIE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ERXIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERXIE` reader - "]
pub struct ERXIE_R(crate::FieldReader<bool>);
impl ERXIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERXIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERXIE_A {
        match self.bits {
            false => ERXIE_A::DISABLE,
            true => ERXIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ERXIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ERXIE_A::ENABLE
    }
}
impl core::ops::Deref for ERXIE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERXIE` writer - "]
pub struct ERXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERXIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERXIE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERXIE_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETXIE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ETXIE_A> for bool {
    #[inline(always)]
    fn from(variant: ETXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETXIE` reader - "]
pub struct ETXIE_R(crate::FieldReader<bool>);
impl ETXIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETXIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETXIE_A {
        match self.bits {
            false => ETXIE_A::DISABLE,
            true => ETXIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ETXIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ETXIE_A::ENABLE
    }
}
impl core::ops::Deref for ETXIE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETXIE` writer - "]
pub struct ETXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETXIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ETXIE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ETXIE_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIT_A {
    #[doc = "0: `0`"]
    MII = 0,
    #[doc = "1: `1`"]
    RGMII = 1,
}
impl From<EPIT_A> for bool {
    #[inline(always)]
    fn from(variant: EPIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIT` reader - "]
pub struct EPIT_R(crate::FieldReader<bool>);
impl EPIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIT_A {
        match self.bits {
            false => EPIT_A::MII,
            true => EPIT_A::RGMII,
        }
    }
    #[doc = "Checks if the value of the field is `MII`"]
    #[inline(always)]
    pub fn is_mii(&self) -> bool {
        **self == EPIT_A::MII
    }
    #[doc = "Checks if the value of the field is `RGMII`"]
    #[inline(always)]
    pub fn is_rgmii(&self) -> bool {
        **self == EPIT_A::RGMII
    }
}
impl core::ops::Deref for EPIT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPIT` writer - "]
pub struct EPIT_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mii(self) -> &'a mut W {
        self.variant(EPIT_A::MII)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rgmii(self) -> &'a mut W {
        self.variant(EPIT_A::RGMII)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETCS_A {
    #[doc = "0: `0`"]
    MII = 0,
    #[doc = "1: `1`"]
    EXTERNAL_GMII_RGMII = 1,
    #[doc = "2: `10`"]
    INTERNAL_GMII_RGMII = 2,
}
impl From<ETCS_A> for u8 {
    #[inline(always)]
    fn from(variant: ETCS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETCS` reader - "]
pub struct ETCS_R(crate::FieldReader<u8>);
impl ETCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ETCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ETCS_A> {
        match self.bits {
            0 => Some(ETCS_A::MII),
            1 => Some(ETCS_A::EXTERNAL_GMII_RGMII),
            2 => Some(ETCS_A::INTERNAL_GMII_RGMII),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MII`"]
    #[inline(always)]
    pub fn is_mii(&self) -> bool {
        **self == ETCS_A::MII
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_GMII_RGMII`"]
    #[inline(always)]
    pub fn is_external_gmii_rgmii(&self) -> bool {
        **self == ETCS_A::EXTERNAL_GMII_RGMII
    }
    #[doc = "Checks if the value of the field is `INTERNAL_GMII_RGMII`"]
    #[inline(always)]
    pub fn is_internal_gmii_rgmii(&self) -> bool {
        **self == ETCS_A::INTERNAL_GMII_RGMII
    }
}
impl core::ops::Deref for ETCS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETCS` writer - "]
pub struct ETCS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETCS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mii(self) -> &'a mut W {
        self.variant(ETCS_A::MII)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_gmii_rgmii(self) -> &'a mut W {
        self.variant(ETCS_A::EXTERNAL_GMII_RGMII)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn internal_gmii_rgmii(self) -> &'a mut W {
        self.variant(ETCS_A::INTERNAL_GMII_RGMII)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bps_efuse(&self) -> BPS_EFUSE_R {
        BPS_EFUSE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn xmii_sel(&self) -> XMII_SEL_R {
        XMII_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn ephy_mode(&self) -> EPHY_MODE_R {
        EPHY_MODE_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn led_pol(&self) -> LED_POL_R {
        LED_POL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn shutdown(&self) -> SHUTDOWN_R {
        SHUTDOWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn phy_select(&self) -> PHY_SELECT_R {
        PHY_SELECT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmii_en(&self) -> RMII_EN_R {
        RMII_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn etxdc(&self) -> ETXDC_R {
        ETXDC_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn erxdc(&self) -> ERXDC_R {
        ERXDC_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn erxie(&self) -> ERXIE_R {
        ERXIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn etxie(&self) -> ETXIE_R {
        ETXIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn epit(&self) -> EPIT_R {
        EPIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn etcs(&self) -> ETCS_R {
        ETCS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bps_efuse(&mut self) -> BPS_EFUSE_W {
        BPS_EFUSE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn xmii_sel(&mut self) -> XMII_SEL_W {
        XMII_SEL_W { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn ephy_mode(&mut self) -> EPHY_MODE_W {
        EPHY_MODE_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn phy_addr(&mut self) -> PHY_ADDR_W {
        PHY_ADDR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W {
        CLK_SEL_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn led_pol(&mut self) -> LED_POL_W {
        LED_POL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn shutdown(&mut self) -> SHUTDOWN_W {
        SHUTDOWN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn phy_select(&mut self) -> PHY_SELECT_W {
        PHY_SELECT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmii_en(&mut self) -> RMII_EN_W {
        RMII_EN_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn etxdc(&mut self) -> ETXDC_W {
        ETXDC_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn erxdc(&mut self) -> ERXDC_W {
        ERXDC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn erxie(&mut self) -> ERXIE_W {
        ERXIE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn etxie(&mut self) -> ETXIE_W {
        ETXIE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn epit(&mut self) -> EPIT_W {
        EPIT_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn etcs(&mut self) -> ETCS_W {
        ETCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC-EPHY Clock Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_ephy_clk0](index.html) module"]
pub struct EMAC_EPHY_CLK0_SPEC;
impl crate::RegisterSpec for EMAC_EPHY_CLK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_ephy_clk0::R](R) reader structure"]
impl crate::Readable for EMAC_EPHY_CLK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_ephy_clk0::W](W) writer structure"]
impl crate::Writable for EMAC_EPHY_CLK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_EPHY_CLK0 to value 0"]
impl crate::Resettable for EMAC_EPHY_CLK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
