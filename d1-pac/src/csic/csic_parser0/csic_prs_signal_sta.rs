#[doc = "Register `csic_prs_signal_sta` reader"]
pub type R = crate::R<CSIC_PRS_SIGNAL_STA_SPEC>;
#[doc = "Register `csic_prs_signal_sta` writer"]
pub type W = crate::W<CSIC_PRS_SIGNAL_STA_SPEC>;
#[doc = "Field `data_sta` reader - Indicates the Dn status (n=0-23), MSB for D23, LSB for D0"]
pub type DATA_STA_R = crate::FieldReader<u32>;
#[doc = "Field `pclk_sta` reader - Indicates the pclk status"]
pub type PCLK_STA_R = crate::BitReader<PCLK_STA_A>;
#[doc = "Indicates the pclk status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCLK_STA_A {
    #[doc = "0: low"]
    LOW = 0,
    #[doc = "1: high"]
    HIGH = 1,
}
impl From<PCLK_STA_A> for bool {
    #[inline(always)]
    fn from(variant: PCLK_STA_A) -> Self {
        variant as u8 != 0
    }
}
impl PCLK_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_STA_A {
        match self.bits {
            false => PCLK_STA_A::LOW,
            true => PCLK_STA_A::HIGH,
        }
    }
    #[doc = "low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PCLK_STA_A::LOW
    }
    #[doc = "high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PCLK_STA_A::HIGH
    }
}
impl R {
    #[doc = "Bits 0:23 - Indicates the Dn status (n=0-23), MSB for D23, LSB for D0"]
    #[inline(always)]
    pub fn data_sta(&self) -> DATA_STA_R {
        DATA_STA_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Indicates the pclk status"]
    #[inline(always)]
    pub fn pclk_sta(&self) -> PCLK_STA_R {
        PCLK_STA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
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
#[doc = "CSIC Parser Signal Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_prs_signal_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_prs_signal_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_PRS_SIGNAL_STA_SPEC;
impl crate::RegisterSpec for CSIC_PRS_SIGNAL_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_prs_signal_sta::R`](R) reader structure"]
impl crate::Readable for CSIC_PRS_SIGNAL_STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_prs_signal_sta::W`](W) writer structure"]
impl crate::Writable for CSIC_PRS_SIGNAL_STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_prs_signal_sta to value 0"]
impl crate::Resettable for CSIC_PRS_SIGNAL_STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
