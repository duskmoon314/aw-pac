#[doc = "Register `emac_ephy_clk0` reader"]
pub type R = crate::R<EMAC_EPHY_CLK0_SPEC>;
#[doc = "Register `emac_ephy_clk0` writer"]
pub type W = crate::W<EMAC_EPHY_CLK0_SPEC>;
#[doc = "Field `etcs` reader - "]
pub type ETCS_R = crate::FieldReader<ETCS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for ETCS_A {
    type Ux = u8;
}
impl ETCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETCS_A> {
        match self.bits {
            0 => Some(ETCS_A::MII),
            1 => Some(ETCS_A::EXTERNAL_GMII_RGMII),
            2 => Some(ETCS_A::INTERNAL_GMII_RGMII),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mii(&self) -> bool {
        *self == ETCS_A::MII
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_external_gmii_rgmii(&self) -> bool {
        *self == ETCS_A::EXTERNAL_GMII_RGMII
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_internal_gmii_rgmii(&self) -> bool {
        *self == ETCS_A::INTERNAL_GMII_RGMII
    }
}
#[doc = "Field `etcs` writer - "]
pub type ETCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ETCS_A>;
impl<'a, REG> ETCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mii(self) -> &'a mut crate::W<REG> {
        self.variant(ETCS_A::MII)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_gmii_rgmii(self) -> &'a mut crate::W<REG> {
        self.variant(ETCS_A::EXTERNAL_GMII_RGMII)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn internal_gmii_rgmii(self) -> &'a mut crate::W<REG> {
        self.variant(ETCS_A::INTERNAL_GMII_RGMII)
    }
}
#[doc = "Field `epit` reader - "]
pub type EPIT_R = crate::BitReader<EPIT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl EPIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPIT_A {
        match self.bits {
            false => EPIT_A::MII,
            true => EPIT_A::RGMII,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mii(&self) -> bool {
        *self == EPIT_A::MII
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_rgmii(&self) -> bool {
        *self == EPIT_A::RGMII
    }
}
#[doc = "Field `epit` writer - "]
pub type EPIT_W<'a, REG> = crate::BitWriter<'a, REG, EPIT_A>;
impl<'a, REG> EPIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mii(self) -> &'a mut crate::W<REG> {
        self.variant(EPIT_A::MII)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rgmii(self) -> &'a mut crate::W<REG> {
        self.variant(EPIT_A::RGMII)
    }
}
#[doc = "Field `etxie` reader - "]
pub type ETXIE_R = crate::BitReader<ETXIE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ETXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETXIE_A {
        match self.bits {
            false => ETXIE_A::DISABLE,
            true => ETXIE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ETXIE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ETXIE_A::ENABLE
    }
}
#[doc = "Field `etxie` writer - "]
pub type ETXIE_W<'a, REG> = crate::BitWriter<'a, REG, ETXIE_A>;
impl<'a, REG> ETXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ETXIE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ETXIE_A::ENABLE)
    }
}
#[doc = "Field `erxie` reader - "]
pub type ERXIE_R = crate::BitReader<ERXIE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ERXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERXIE_A {
        match self.bits {
            false => ERXIE_A::DISABLE,
            true => ERXIE_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERXIE_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERXIE_A::ENABLE
    }
}
#[doc = "Field `erxie` writer - "]
pub type ERXIE_W<'a, REG> = crate::BitWriter<'a, REG, ERXIE_A>;
impl<'a, REG> ERXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ERXIE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ERXIE_A::ENABLE)
    }
}
#[doc = "Field `erxdc` reader - "]
pub type ERXDC_R = crate::FieldReader;
#[doc = "Field `erxdc` writer - "]
pub type ERXDC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `etxdc` reader - "]
pub type ETXDC_R = crate::FieldReader;
#[doc = "Field `etxdc` writer - "]
pub type ETXDC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rmii_en` reader - "]
pub type RMII_EN_R = crate::BitReader<RMII_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RMII_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RMII_EN_A {
        match self.bits {
            false => RMII_EN_A::DISABLE,
            true => RMII_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RMII_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RMII_EN_A::ENABLE
    }
}
#[doc = "Field `rmii_en` writer - "]
pub type RMII_EN_W<'a, REG> = crate::BitWriter<'a, REG, RMII_EN_A>;
impl<'a, REG> RMII_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RMII_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RMII_EN_A::ENABLE)
    }
}
#[doc = "Field `phy_select` reader - "]
pub type PHY_SELECT_R = crate::BitReader<PHY_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PHY_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PHY_SELECT_A {
        match self.bits {
            false => PHY_SELECT_A::EXTERNAL,
            true => PHY_SELECT_A::INTERNAL,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == PHY_SELECT_A::EXTERNAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == PHY_SELECT_A::INTERNAL
    }
}
#[doc = "Field `phy_select` writer - "]
pub type PHY_SELECT_W<'a, REG> = crate::BitWriter<'a, REG, PHY_SELECT_A>;
impl<'a, REG> PHY_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(PHY_SELECT_A::EXTERNAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(PHY_SELECT_A::INTERNAL)
    }
}
#[doc = "Field `shutdown` reader - "]
pub type SHUTDOWN_R = crate::BitReader<SHUTDOWN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SHUTDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SHUTDOWN_A {
        match self.bits {
            false => SHUTDOWN_A::POWER_UP,
            true => SHUTDOWN_A::SHUT_DOWN,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        *self == SHUTDOWN_A::POWER_UP
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_shut_down(&self) -> bool {
        *self == SHUTDOWN_A::SHUT_DOWN
    }
}
#[doc = "Field `shutdown` writer - "]
pub type SHUTDOWN_W<'a, REG> = crate::BitWriter<'a, REG, SHUTDOWN_A>;
impl<'a, REG> SHUTDOWN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut crate::W<REG> {
        self.variant(SHUTDOWN_A::POWER_UP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn shut_down(self) -> &'a mut crate::W<REG> {
        self.variant(SHUTDOWN_A::SHUT_DOWN)
    }
}
#[doc = "Field `led_pol` reader - "]
pub type LED_POL_R = crate::BitReader<LED_POL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LED_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LED_POL_A {
        match self.bits {
            false => LED_POL_A::HIGH,
            true => LED_POL_A::LOW,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LED_POL_A::HIGH
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LED_POL_A::LOW
    }
}
#[doc = "Field `led_pol` writer - "]
pub type LED_POL_W<'a, REG> = crate::BitWriter<'a, REG, LED_POL_A>;
impl<'a, REG> LED_POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(LED_POL_A::HIGH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(LED_POL_A::LOW)
    }
}
#[doc = "Field `clk_sel` reader - "]
pub type CLK_SEL_R = crate::BitReader<CLK_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_SEL_A {
        match self.bits {
            false => CLK_SEL_A::F25M,
            true => CLK_SEL_A::F24M,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_f25m(&self) -> bool {
        *self == CLK_SEL_A::F25M
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_f24m(&self) -> bool {
        *self == CLK_SEL_A::F24M
    }
}
#[doc = "Field `clk_sel` writer - "]
pub type CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG, CLK_SEL_A>;
impl<'a, REG> CLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn f25m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SEL_A::F25M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn f24m(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SEL_A::F24M)
    }
}
#[doc = "Field `phy_addr` reader - "]
pub type PHY_ADDR_R = crate::FieldReader;
#[doc = "Field `phy_addr` writer - "]
pub type PHY_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ephy_mode` reader - "]
pub type EPHY_MODE_R = crate::FieldReader<EPHY_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for EPHY_MODE_A {
    type Ux = u8;
}
impl EPHY_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EPHY_MODE_A> {
        match self.bits {
            0 => Some(EPHY_MODE_A::NORMAL),
            1 => Some(EPHY_MODE_A::SIMULATION),
            2 => Some(EPHY_MODE_A::AFE_TEST),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == EPHY_MODE_A::NORMAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_simulation(&self) -> bool {
        *self == EPHY_MODE_A::SIMULATION
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_afe_test(&self) -> bool {
        *self == EPHY_MODE_A::AFE_TEST
    }
}
#[doc = "Field `ephy_mode` writer - "]
pub type EPHY_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EPHY_MODE_A>;
impl<'a, REG> EPHY_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(EPHY_MODE_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn simulation(self) -> &'a mut crate::W<REG> {
        self.variant(EPHY_MODE_A::SIMULATION)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn afe_test(self) -> &'a mut crate::W<REG> {
        self.variant(EPHY_MODE_A::AFE_TEST)
    }
}
#[doc = "Field `xmii_sel` reader - "]
pub type XMII_SEL_R = crate::BitReader<XMII_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl XMII_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XMII_SEL_A {
        match self.bits {
            false => XMII_SEL_A::INTERNAL,
            true => XMII_SEL_A::EXTERNAL,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == XMII_SEL_A::INTERNAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == XMII_SEL_A::EXTERNAL
    }
}
#[doc = "Field `xmii_sel` writer - "]
pub type XMII_SEL_W<'a, REG> = crate::BitWriter<'a, REG, XMII_SEL_A>;
impl<'a, REG> XMII_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(XMII_SEL_A::INTERNAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(XMII_SEL_A::EXTERNAL)
    }
}
#[doc = "Field `bps_efuse` reader - "]
pub type BPS_EFUSE_R = crate::FieldReader;
#[doc = "Field `bps_efuse` writer - "]
pub type BPS_EFUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn etcs(&self) -> ETCS_R {
        ETCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn epit(&self) -> EPIT_R {
        EPIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn etxie(&self) -> ETXIE_R {
        ETXIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn erxie(&self) -> ERXIE_R {
        ERXIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn erxdc(&self) -> ERXDC_R {
        ERXDC_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn etxdc(&self) -> ETXDC_R {
        ETXDC_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmii_en(&self) -> RMII_EN_R {
        RMII_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn phy_select(&self) -> PHY_SELECT_R {
        PHY_SELECT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn shutdown(&self) -> SHUTDOWN_R {
        SHUTDOWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn led_pol(&self) -> LED_POL_R {
        LED_POL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn ephy_mode(&self) -> EPHY_MODE_R {
        EPHY_MODE_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn xmii_sel(&self) -> XMII_SEL_R {
        XMII_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bps_efuse(&self) -> BPS_EFUSE_R {
        BPS_EFUSE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn etcs(&mut self) -> ETCS_W<EMAC_EPHY_CLK0_SPEC> {
        ETCS_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn epit(&mut self) -> EPIT_W<EMAC_EPHY_CLK0_SPEC> {
        EPIT_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn etxie(&mut self) -> ETXIE_W<EMAC_EPHY_CLK0_SPEC> {
        ETXIE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn erxie(&mut self) -> ERXIE_W<EMAC_EPHY_CLK0_SPEC> {
        ERXIE_W::new(self, 4)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    #[must_use]
    pub fn erxdc(&mut self) -> ERXDC_W<EMAC_EPHY_CLK0_SPEC> {
        ERXDC_W::new(self, 5)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    #[must_use]
    pub fn etxdc(&mut self) -> ETXDC_W<EMAC_EPHY_CLK0_SPEC> {
        ETXDC_W::new(self, 10)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn rmii_en(&mut self) -> RMII_EN_W<EMAC_EPHY_CLK0_SPEC> {
        RMII_EN_W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn phy_select(&mut self) -> PHY_SELECT_W<EMAC_EPHY_CLK0_SPEC> {
        PHY_SELECT_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn shutdown(&mut self) -> SHUTDOWN_W<EMAC_EPHY_CLK0_SPEC> {
        SHUTDOWN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn led_pol(&mut self) -> LED_POL_W<EMAC_EPHY_CLK0_SPEC> {
        LED_POL_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<EMAC_EPHY_CLK0_SPEC> {
        CLK_SEL_W::new(self, 18)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn phy_addr(&mut self) -> PHY_ADDR_W<EMAC_EPHY_CLK0_SPEC> {
        PHY_ADDR_W::new(self, 20)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    #[must_use]
    pub fn ephy_mode(&mut self) -> EPHY_MODE_W<EMAC_EPHY_CLK0_SPEC> {
        EPHY_MODE_W::new(self, 25)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn xmii_sel(&mut self) -> XMII_SEL_W<EMAC_EPHY_CLK0_SPEC> {
        XMII_SEL_W::new(self, 27)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn bps_efuse(&mut self) -> BPS_EFUSE_W<EMAC_EPHY_CLK0_SPEC> {
        BPS_EFUSE_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EMAC-EPHY Clock Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_ephy_clk0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_ephy_clk0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_EPHY_CLK0_SPEC;
impl crate::RegisterSpec for EMAC_EPHY_CLK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_ephy_clk0::R`](R) reader structure"]
impl crate::Readable for EMAC_EPHY_CLK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_ephy_clk0::W`](W) writer structure"]
impl crate::Writable for EMAC_EPHY_CLK0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_ephy_clk0 to value 0"]
impl crate::Resettable for EMAC_EPHY_CLK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
