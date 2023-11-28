#[doc = "Register `ths_filter` reader"]
pub type R = crate::R<THS_FILTER_SPEC>;
#[doc = "Register `ths_filter` writer"]
pub type W = crate::W<THS_FILTER_SPEC>;
#[doc = "Field `filter_type` reader - Averaging filter type"]
pub type FILTER_TYPE_R = crate::FieldReader<FILTER_TYPE_A>;
#[doc = "Averaging filter type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTER_TYPE_A {
    #[doc = "0: 2"]
    T2 = 0,
    #[doc = "1: 4"]
    T4 = 1,
    #[doc = "2: 8"]
    T8 = 2,
    #[doc = "3: 16"]
    T16 = 3,
}
impl From<FILTER_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_TYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FILTER_TYPE_A {
    type Ux = u8;
}
impl FILTER_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FILTER_TYPE_A {
        match self.bits {
            0 => FILTER_TYPE_A::T2,
            1 => FILTER_TYPE_A::T4,
            2 => FILTER_TYPE_A::T8,
            3 => FILTER_TYPE_A::T16,
            _ => unreachable!(),
        }
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_t2(&self) -> bool {
        *self == FILTER_TYPE_A::T2
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_t4(&self) -> bool {
        *self == FILTER_TYPE_A::T4
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_t8(&self) -> bool {
        *self == FILTER_TYPE_A::T8
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_t16(&self) -> bool {
        *self == FILTER_TYPE_A::T16
    }
}
#[doc = "Field `filter_type` writer - Averaging filter type"]
pub type FILTER_TYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FILTER_TYPE_A>;
impl<'a, REG> FILTER_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2"]
    #[inline(always)]
    pub fn t2(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_TYPE_A::T2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn t4(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_TYPE_A::T4)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn t8(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_TYPE_A::T8)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn t16(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_TYPE_A::T16)
    }
}
#[doc = "Field `filter_en` reader - Filter enable"]
pub type FILTER_EN_R = crate::BitReader<FILTER_EN_A>;
#[doc = "Filter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILTER_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FILTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FILTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FILTER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FILTER_EN_A {
        match self.bits {
            false => FILTER_EN_A::DISABLE,
            true => FILTER_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FILTER_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FILTER_EN_A::ENABLE
    }
}
#[doc = "Field `filter_en` writer - Filter enable"]
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG, FILTER_EN_A>;
impl<'a, REG> FILTER_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Averaging filter type"]
    #[inline(always)]
    pub fn filter_type(&self) -> FILTER_TYPE_R {
        FILTER_TYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Filter enable"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Averaging filter type"]
    #[inline(always)]
    #[must_use]
    pub fn filter_type(&mut self) -> FILTER_TYPE_W<THS_FILTER_SPEC> {
        FILTER_TYPE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<THS_FILTER_SPEC> {
        FILTER_EN_W::new(self, 2)
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
#[doc = "THS Median Filter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_filter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_filter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_FILTER_SPEC;
impl crate::RegisterSpec for THS_FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_filter::R`](R) reader structure"]
impl crate::Readable for THS_FILTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_filter::W`](W) writer structure"]
impl crate::Writable for THS_FILTER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_filter to value 0x01"]
impl crate::Resettable for THS_FILTER_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
