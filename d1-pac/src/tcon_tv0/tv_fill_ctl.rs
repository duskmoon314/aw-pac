#[doc = "Register `tv_fill_ctl` reader"]
pub type R = crate::R<TV_FILL_CTL_SPEC>;
#[doc = "Register `tv_fill_ctl` writer"]
pub type W = crate::W<TV_FILL_CTL_SPEC>;
#[doc = "Field `tv_fill_en` reader - TV Fill Enable"]
pub type TV_FILL_EN_R = crate::BitReader<TV_FILL_EN_A>;
#[doc = "TV Fill Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TV_FILL_EN_A {
    #[doc = "0: Bypass"]
    BYPASS = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TV_FILL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TV_FILL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TV_FILL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TV_FILL_EN_A {
        match self.bits {
            false => TV_FILL_EN_A::BYPASS,
            true => TV_FILL_EN_A::ENABLE,
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == TV_FILL_EN_A::BYPASS
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TV_FILL_EN_A::ENABLE
    }
}
#[doc = "Field `tv_fill_en` writer - TV Fill Enable"]
pub type TV_FILL_EN_W<'a, REG> = crate::BitWriter<'a, REG, TV_FILL_EN_A>;
impl<'a, REG> TV_FILL_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(TV_FILL_EN_A::BYPASS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TV_FILL_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 31 - TV Fill Enable"]
    #[inline(always)]
    pub fn tv_fill_en(&self) -> TV_FILL_EN_R {
        TV_FILL_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - TV Fill Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tv_fill_en(&mut self) -> TV_FILL_EN_W<TV_FILL_CTL_SPEC> {
        TV_FILL_EN_W::new(self, 31)
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
#[doc = "TV Fill Data Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_fill_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_fill_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_FILL_CTL_SPEC;
impl crate::RegisterSpec for TV_FILL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_fill_ctl::R`](R) reader structure"]
impl crate::Readable for TV_FILL_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_fill_ctl::W`](W) writer structure"]
impl crate::Writable for TV_FILL_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_fill_ctl to value 0"]
impl crate::Resettable for TV_FILL_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
