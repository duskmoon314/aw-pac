#[doc = "Register `tv_ceu_ctl` reader"]
pub type R = crate::R<TV_CEU_CTL_SPEC>;
#[doc = "Register `tv_ceu_ctl` writer"]
pub type W = crate::W<TV_CEU_CTL_SPEC>;
#[doc = "Field `ceu_en` reader - CEU Enable"]
pub type CEU_EN_R = crate::BitReader<CEU_EN_A>;
#[doc = "CEU Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEU_EN_A {
    #[doc = "0: Bypass"]
    BYPASS = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CEU_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CEU_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CEU_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEU_EN_A {
        match self.bits {
            false => CEU_EN_A::BYPASS,
            true => CEU_EN_A::ENABLE,
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == CEU_EN_A::BYPASS
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CEU_EN_A::ENABLE
    }
}
#[doc = "Field `ceu_en` writer - CEU Enable"]
pub type CEU_EN_W<'a, REG> = crate::BitWriter<'a, REG, CEU_EN_A>;
impl<'a, REG> CEU_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(CEU_EN_A::BYPASS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CEU_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 31 - CEU Enable"]
    #[inline(always)]
    pub fn ceu_en(&self) -> CEU_EN_R {
        CEU_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - CEU Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ceu_en(&mut self) -> CEU_EN_W<TV_CEU_CTL_SPEC> {
        CEU_EN_W::new(self, 31)
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
#[doc = "TV CEU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_ceu_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_ceu_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_CEU_CTL_SPEC;
impl crate::RegisterSpec for TV_CEU_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_ceu_ctl::R`](R) reader structure"]
impl crate::Readable for TV_CEU_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_ceu_ctl::W`](W) writer structure"]
impl crate::Writable for TV_CEU_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_ceu_ctl to value 0"]
impl crate::Resettable for TV_CEU_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
