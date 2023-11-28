#[doc = "Register `dbi_timer` reader"]
pub type R = crate::R<DBI_TIMER_SPEC>;
#[doc = "Register `dbi_timer` writer"]
pub type W = crate::W<DBI_TIMER_SPEC>;
#[doc = "Field `dbi_timer_value` reader - "]
pub type DBI_TIMER_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `dbi_timer_value` writer - "]
pub type DBI_TIMER_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `dbi_tm_en` reader - DBI Timer Enable"]
pub type DBI_TM_EN_R = crate::BitReader<DBI_TM_EN_A>;
#[doc = "DBI Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBI_TM_EN_A {
    #[doc = "0: `0`"]
    ENABLE = 0,
    #[doc = "1: `1`"]
    DISABLE = 1,
}
impl From<DBI_TM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DBI_TM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBI_TM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBI_TM_EN_A {
        match self.bits {
            false => DBI_TM_EN_A::ENABLE,
            true => DBI_TM_EN_A::DISABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBI_TM_EN_A::ENABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBI_TM_EN_A::DISABLE
    }
}
#[doc = "Field `dbi_tm_en` writer - DBI Timer Enable"]
pub type DBI_TM_EN_W<'a, REG> = crate::BitWriter<'a, REG, DBI_TM_EN_A>;
impl<'a, REG> DBI_TM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DBI_TM_EN_A::ENABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DBI_TM_EN_A::DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn dbi_timer_value(&self) -> DBI_TIMER_VALUE_R {
        DBI_TIMER_VALUE_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - DBI Timer Enable"]
    #[inline(always)]
    pub fn dbi_tm_en(&self) -> DBI_TM_EN_R {
        DBI_TM_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_timer_value(&mut self) -> DBI_TIMER_VALUE_W<DBI_TIMER_SPEC> {
        DBI_TIMER_VALUE_W::new(self, 0)
    }
    #[doc = "Bit 31 - DBI Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_tm_en(&mut self) -> DBI_TM_EN_W<DBI_TIMER_SPEC> {
        DBI_TM_EN_W::new(self, 31)
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
#[doc = "DBI Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbi_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_TIMER_SPEC;
impl crate::RegisterSpec for DBI_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_timer::R`](R) reader structure"]
impl crate::Readable for DBI_TIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbi_timer::W`](W) writer structure"]
impl crate::Writable for DBI_TIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dbi_timer to value 0"]
impl crate::Resettable for DBI_TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
