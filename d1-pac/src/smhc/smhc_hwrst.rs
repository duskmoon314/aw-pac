#[doc = "Register `smhc_hwrst` reader"]
pub type R = crate::R<SMHC_HWRST_SPEC>;
#[doc = "Register `smhc_hwrst` writer"]
pub type W = crate::W<SMHC_HWRST_SPEC>;
#[doc = "Field `hw_rst` reader - "]
pub type HW_RST_R = crate::BitReader<HW_RST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HW_RST_A {
    #[doc = "0: Active mode"]
    ACTIVE = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<HW_RST_A> for bool {
    #[inline(always)]
    fn from(variant: HW_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl HW_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HW_RST_A {
        match self.bits {
            false => HW_RST_A::ACTIVE,
            true => HW_RST_A::RESET,
        }
    }
    #[doc = "Active mode"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HW_RST_A::ACTIVE
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == HW_RST_A::RESET
    }
}
#[doc = "Field `hw_rst` writer - "]
pub type HW_RST_W<'a, REG> = crate::BitWriter<'a, REG, HW_RST_A>;
impl<'a, REG> HW_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(HW_RST_A::ACTIVE)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(HW_RST_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hw_rst(&self) -> HW_RST_R {
        HW_RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hw_rst(&mut self) -> HW_RST_W<SMHC_HWRST_SPEC> {
        HW_RST_W::new(self, 0)
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
#[doc = "Hardware Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_hwrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_hwrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_HWRST_SPEC;
impl crate::RegisterSpec for SMHC_HWRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_hwrst::R`](R) reader structure"]
impl crate::Readable for SMHC_HWRST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_hwrst::W`](W) writer structure"]
impl crate::Writable for SMHC_HWRST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_hwrst to value 0"]
impl crate::Resettable for SMHC_HWRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
