#[doc = "Register `emac_rx_ctl0` reader"]
pub type R = crate::R<EMAC_RX_CTL0_SPEC>;
#[doc = "Register `emac_rx_ctl0` writer"]
pub type W = crate::W<EMAC_RX_CTL0_SPEC>;
#[doc = "Field `rx_flow_ctl_en` reader - "]
pub type RX_FLOW_CTL_EN_R = crate::BitReader;
#[doc = "Field `rx_flow_ctl_en` writer - "]
pub type RX_FLOW_CTL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_pause_frm_md` reader - "]
pub type RX_PAUSE_FRM_MD_R = crate::BitReader<RX_PAUSE_FRM_MD_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_PAUSE_FRM_MD_A {
    #[doc = "0: `0`"]
    ONLY_MULTICAST = 0,
    #[doc = "1: `1`"]
    ALSO_UNICAST_MAC0 = 1,
}
impl From<RX_PAUSE_FRM_MD_A> for bool {
    #[inline(always)]
    fn from(variant: RX_PAUSE_FRM_MD_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_PAUSE_FRM_MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_PAUSE_FRM_MD_A {
        match self.bits {
            false => RX_PAUSE_FRM_MD_A::ONLY_MULTICAST,
            true => RX_PAUSE_FRM_MD_A::ALSO_UNICAST_MAC0,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_only_multicast(&self) -> bool {
        *self == RX_PAUSE_FRM_MD_A::ONLY_MULTICAST
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_also_unicast_mac0(&self) -> bool {
        *self == RX_PAUSE_FRM_MD_A::ALSO_UNICAST_MAC0
    }
}
#[doc = "Field `rx_pause_frm_md` writer - "]
pub type RX_PAUSE_FRM_MD_W<'a, REG> = crate::BitWriter<'a, REG, RX_PAUSE_FRM_MD_A>;
impl<'a, REG> RX_PAUSE_FRM_MD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn only_multicast(self) -> &'a mut crate::W<REG> {
        self.variant(RX_PAUSE_FRM_MD_A::ONLY_MULTICAST)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn also_unicast_mac0(self) -> &'a mut crate::W<REG> {
        self.variant(RX_PAUSE_FRM_MD_A::ALSO_UNICAST_MAC0)
    }
}
#[doc = "Field `check_crc` reader - Check CRC Enable"]
pub type CHECK_CRC_R = crate::BitReader<CHECK_CRC_A>;
#[doc = "Check CRC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHECK_CRC_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    CHECK = 1,
}
impl From<CHECK_CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CHECK_CRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CHECK_CRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHECK_CRC_A {
        match self.bits {
            false => CHECK_CRC_A::DISABLE,
            true => CHECK_CRC_A::CHECK,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHECK_CRC_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_check(&self) -> bool {
        *self == CHECK_CRC_A::CHECK
    }
}
#[doc = "Field `check_crc` writer - Check CRC Enable"]
pub type CHECK_CRC_W<'a, REG> = crate::BitWriter<'a, REG, CHECK_CRC_A>;
impl<'a, REG> CHECK_CRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CHECK_CRC_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn check(self) -> &'a mut crate::W<REG> {
        self.variant(CHECK_CRC_A::CHECK)
    }
}
#[doc = "Field `strip_fcs` reader - "]
pub type STRIP_FCS_R = crate::BitReader;
#[doc = "Field `strip_fcs` writer - "]
pub type STRIP_FCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `jumbo_frm_en` reader - Jumbo Frame Enable"]
pub type JUMBO_FRM_EN_R = crate::BitReader<JUMBO_FRM_EN_A>;
#[doc = "Jumbo Frame Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JUMBO_FRM_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<JUMBO_FRM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: JUMBO_FRM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl JUMBO_FRM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JUMBO_FRM_EN_A {
        match self.bits {
            false => JUMBO_FRM_EN_A::DISABLE,
            true => JUMBO_FRM_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == JUMBO_FRM_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == JUMBO_FRM_EN_A::ENABLE
    }
}
#[doc = "Field `jumbo_frm_en` writer - Jumbo Frame Enable"]
pub type JUMBO_FRM_EN_W<'a, REG> = crate::BitWriter<'a, REG, JUMBO_FRM_EN_A>;
impl<'a, REG> JUMBO_FRM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(JUMBO_FRM_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(JUMBO_FRM_EN_A::ENABLE)
    }
}
#[doc = "Field `rx_frm_len_ctl` reader - Frame Receive Length Control"]
pub type RX_FRM_LEN_CTL_R = crate::BitReader<RX_FRM_LEN_CTL_A>;
#[doc = "Frame Receive Length Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FRM_LEN_CTL_A {
    #[doc = "0: `0`"]
    B2048 = 0,
    #[doc = "1: `1`"]
    B16384 = 1,
}
impl From<RX_FRM_LEN_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FRM_LEN_CTL_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FRM_LEN_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_FRM_LEN_CTL_A {
        match self.bits {
            false => RX_FRM_LEN_CTL_A::B2048,
            true => RX_FRM_LEN_CTL_A::B16384,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_b2048(&self) -> bool {
        *self == RX_FRM_LEN_CTL_A::B2048
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_b16384(&self) -> bool {
        *self == RX_FRM_LEN_CTL_A::B16384
    }
}
#[doc = "Field `rx_frm_len_ctl` writer - Frame Receive Length Control"]
pub type RX_FRM_LEN_CTL_W<'a, REG> = crate::BitWriter<'a, REG, RX_FRM_LEN_CTL_A>;
impl<'a, REG> RX_FRM_LEN_CTL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn b2048(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FRM_LEN_CTL_A::B2048)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn b16384(self) -> &'a mut crate::W<REG> {
        self.variant(RX_FRM_LEN_CTL_A::B16384)
    }
}
#[doc = "Field `rx_en` reader - Enable Receiver"]
pub type RX_EN_R = crate::BitReader<RX_EN_A>;
#[doc = "Enable Receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_EN_A {
        match self.bits {
            false => RX_EN_A::DISABLE,
            true => RX_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_EN_A::ENABLE
    }
}
#[doc = "Field `rx_en` writer - Enable Receiver"]
pub type RX_EN_W<'a, REG> = crate::BitWriter<'a, REG, RX_EN_A>;
impl<'a, REG> RX_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RX_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_flow_ctl_en(&self) -> RX_FLOW_CTL_EN_R {
        RX_FLOW_CTL_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_pause_frm_md(&self) -> RX_PAUSE_FRM_MD_R {
        RX_PAUSE_FRM_MD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 27 - Check CRC Enable"]
    #[inline(always)]
    pub fn check_crc(&self) -> CHECK_CRC_R {
        CHECK_CRC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn strip_fcs(&self) -> STRIP_FCS_R {
        STRIP_FCS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn jumbo_frm_en(&self) -> JUMBO_FRM_EN_R {
        JUMBO_FRM_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Frame Receive Length Control"]
    #[inline(always)]
    pub fn rx_frm_len_ctl(&self) -> RX_FRM_LEN_CTL_R {
        RX_FRM_LEN_CTL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Receiver"]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_ctl_en(&mut self) -> RX_FLOW_CTL_EN_W<EMAC_RX_CTL0_SPEC> {
        RX_FLOW_CTL_EN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pause_frm_md(&mut self) -> RX_PAUSE_FRM_MD_W<EMAC_RX_CTL0_SPEC> {
        RX_PAUSE_FRM_MD_W::new(self, 17)
    }
    #[doc = "Bit 27 - Check CRC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn check_crc(&mut self) -> CHECK_CRC_W<EMAC_RX_CTL0_SPEC> {
        CHECK_CRC_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn strip_fcs(&mut self) -> STRIP_FCS_W<EMAC_RX_CTL0_SPEC> {
        STRIP_FCS_W::new(self, 28)
    }
    #[doc = "Bit 29 - Jumbo Frame Enable"]
    #[inline(always)]
    #[must_use]
    pub fn jumbo_frm_en(&mut self) -> JUMBO_FRM_EN_W<EMAC_RX_CTL0_SPEC> {
        JUMBO_FRM_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Frame Receive Length Control"]
    #[inline(always)]
    #[must_use]
    pub fn rx_frm_len_ctl(&mut self) -> RX_FRM_LEN_CTL_W<EMAC_RX_CTL0_SPEC> {
        RX_FRM_LEN_CTL_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rx_en(&mut self) -> RX_EN_W<EMAC_RX_CTL0_SPEC> {
        RX_EN_W::new(self, 31)
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
#[doc = "EMAC Receive Control Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_rx_ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_rx_ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_RX_CTL0_SPEC;
impl crate::RegisterSpec for EMAC_RX_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_rx_ctl0::R`](R) reader structure"]
impl crate::Readable for EMAC_RX_CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_rx_ctl0::W`](W) writer structure"]
impl crate::Writable for EMAC_RX_CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_rx_ctl0 to value 0"]
impl crate::Resettable for EMAC_RX_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
