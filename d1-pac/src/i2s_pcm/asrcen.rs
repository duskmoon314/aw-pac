#[doc = "Register `asrcen` reader"]
pub type R = crate::R<ASRCEN_SPEC>;
#[doc = "Register `asrcen` writer"]
pub type W = crate::W<ASRCEN_SPEC>;
#[doc = "Field `asrc_fn` reader - ASRC Function Enable"]
pub type ASRC_FN_R = crate::BitReader<ASRC_FN_A>;
#[doc = "ASRC Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASRC_FN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ASRC_FN_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_FN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASRC_FN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASRC_FN_A {
        match self.bits {
            false => ASRC_FN_A::DISABLE,
            true => ASRC_FN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ASRC_FN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ASRC_FN_A::ENABLE
    }
}
#[doc = "Field `asrc_fn` writer - ASRC Function Enable"]
pub type ASRC_FN_W<'a, REG> = crate::BitWriter<'a, REG, ASRC_FN_A>;
impl<'a, REG> ASRC_FN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ASRC_FN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ASRC_FN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 31 - ASRC Function Enable"]
    #[inline(always)]
    pub fn asrc_fn(&self) -> ASRC_FN_R {
        ASRC_FN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - ASRC Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asrc_fn(&mut self) -> ASRC_FN_W<ASRCEN_SPEC> {
        ASRC_FN_W::new(self, 31)
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
#[doc = "ASRC Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASRCEN_SPEC;
impl crate::RegisterSpec for ASRCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asrcen::R`](R) reader structure"]
impl crate::Readable for ASRCEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asrcen::W`](W) writer structure"]
impl crate::Writable for ASRCEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets asrcen to value 0"]
impl crate::Resettable for ASRCEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
