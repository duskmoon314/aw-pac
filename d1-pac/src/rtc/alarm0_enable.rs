#[doc = "Register `alarm0_enable` reader"]
pub type R = crate::R<ALARM0_ENABLE_SPEC>;
#[doc = "Register `alarm0_enable` writer"]
pub type W = crate::W<ALARM0_ENABLE_SPEC>;
#[doc = "Field `alm_0_en` reader - Alarm 0 Enable"]
pub type ALM_0_EN_R = crate::BitReader<ALM_0_EN_A>;
#[doc = "Alarm 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALM_0_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ALM_0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ALM_0_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALM_0_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALM_0_EN_A {
        match self.bits {
            false => ALM_0_EN_A::DISABLE,
            true => ALM_0_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALM_0_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALM_0_EN_A::ENABLE
    }
}
#[doc = "Field `alm_0_en` writer - Alarm 0 Enable"]
pub type ALM_0_EN_W<'a, REG> = crate::BitWriter<'a, REG, ALM_0_EN_A>;
impl<'a, REG> ALM_0_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ALM_0_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ALM_0_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm 0 Enable"]
    #[inline(always)]
    pub fn alm_0_en(&self) -> ALM_0_EN_R {
        ALM_0_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alm_0_en(&mut self) -> ALM_0_EN_W<ALARM0_ENABLE_SPEC> {
        ALM_0_EN_W::new(self, 0)
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
#[doc = "Alarm 0 Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM0_ENABLE_SPEC;
impl crate::RegisterSpec for ALARM0_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0_enable::R`](R) reader structure"]
impl crate::Readable for ALARM0_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm0_enable::W`](W) writer structure"]
impl crate::Writable for ALARM0_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets alarm0_enable to value 0"]
impl crate::Resettable for ALARM0_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
