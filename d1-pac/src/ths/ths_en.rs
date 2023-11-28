#[doc = "Register `ths_en` reader"]
pub type R = crate::R<THS_EN_SPEC>;
#[doc = "Register `ths_en` writer"]
pub type W = crate::W<THS_EN_SPEC>;
#[doc = "Field `ths_en` reader - Enable temperature measurement sensor"]
pub type THS_EN_R = crate::BitReader<THS_EN_A>;
#[doc = "Enable temperature measurement sensor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THS_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<THS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: THS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl THS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THS_EN_A {
        match self.bits {
            false => THS_EN_A::DISABLE,
            true => THS_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == THS_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == THS_EN_A::ENABLE
    }
}
#[doc = "Field `ths_en` writer - Enable temperature measurement sensor"]
pub type THS_EN_W<'a, REG> = crate::BitWriter<'a, REG, THS_EN_A>;
impl<'a, REG> THS_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(THS_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(THS_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable temperature measurement sensor"]
    #[inline(always)]
    pub fn ths_en(&self) -> THS_EN_R {
        THS_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable temperature measurement sensor"]
    #[inline(always)]
    #[must_use]
    pub fn ths_en(&mut self) -> THS_EN_W<THS_EN_SPEC> {
        THS_EN_W::new(self, 0)
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
#[doc = "THS Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_EN_SPEC;
impl crate::RegisterSpec for THS_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_en::R`](R) reader structure"]
impl crate::Readable for THS_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_en::W`](W) writer structure"]
impl crate::Writable for THS_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_en to value 0"]
impl crate::Resettable for THS_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
