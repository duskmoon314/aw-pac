#[doc = "Register `ts_tmode_sel` reader"]
pub type R = crate::R<TS_TMODE_SEL_SPEC>;
#[doc = "Register `ts_tmode_sel` writer"]
pub type W = crate::W<TS_TMODE_SEL_SPEC>;
#[doc = "Field `ts_test_mode_en` reader - Timestamp Test Mode Enable"]
pub type TS_TEST_MODE_EN_R = crate::BitReader<TS_TEST_MODE_EN_A>;
#[doc = "Timestamp Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS_TEST_MODE_EN_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    TEST = 1,
}
impl From<TS_TEST_MODE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TS_TEST_MODE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TS_TEST_MODE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS_TEST_MODE_EN_A {
        match self.bits {
            false => TS_TEST_MODE_EN_A::NORMAL,
            true => TS_TEST_MODE_EN_A::TEST,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TS_TEST_MODE_EN_A::NORMAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == TS_TEST_MODE_EN_A::TEST
    }
}
#[doc = "Field `ts_test_mode_en` writer - Timestamp Test Mode Enable"]
pub type TS_TEST_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG, TS_TEST_MODE_EN_A>;
impl<'a, REG> TS_TEST_MODE_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(TS_TEST_MODE_EN_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn test(self) -> &'a mut crate::W<REG> {
        self.variant(TS_TEST_MODE_EN_A::TEST)
    }
}
impl R {
    #[doc = "Bit 0 - Timestamp Test Mode Enable"]
    #[inline(always)]
    pub fn ts_test_mode_en(&self) -> TS_TEST_MODE_EN_R {
        TS_TEST_MODE_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts_test_mode_en(&mut self) -> TS_TEST_MODE_EN_W<TS_TMODE_SEL_SPEC> {
        TS_TEST_MODE_EN_W::new(self, 0)
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
#[doc = "Timestamp Test Mode Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts_tmode_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts_tmode_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TS_TMODE_SEL_SPEC;
impl crate::RegisterSpec for TS_TMODE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_tmode_sel::R`](R) reader structure"]
impl crate::Readable for TS_TMODE_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ts_tmode_sel::W`](W) writer structure"]
impl crate::Writable for TS_TMODE_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ts_tmode_sel to value 0"]
impl crate::Resettable for TS_TMODE_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
