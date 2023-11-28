#[doc = "Register `tp_ctrl3` reader"]
pub type R = crate::R<TP_CTRL3_SPEC>;
#[doc = "Register `tp_ctrl3` writer"]
pub type W = crate::W<TP_CTRL3_SPEC>;
#[doc = "Field `filter_type` reader - Filter Type"]
pub type FILTER_TYPE_R = crate::FieldReader<FILTER_TYPE_A>;
#[doc = "Filter Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTER_TYPE_A {
    #[doc = "0: 4 / 2"]
    T42 = 0,
    #[doc = "1: 5 / 3"]
    T53 = 1,
    #[doc = "2: 8 / 4"]
    T84 = 2,
    #[doc = "3: 16 / 8"]
    T168 = 3,
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
            0 => FILTER_TYPE_A::T42,
            1 => FILTER_TYPE_A::T53,
            2 => FILTER_TYPE_A::T84,
            3 => FILTER_TYPE_A::T168,
            _ => unreachable!(),
        }
    }
    #[doc = "4 / 2"]
    #[inline(always)]
    pub fn is_t42(&self) -> bool {
        *self == FILTER_TYPE_A::T42
    }
    #[doc = "5 / 3"]
    #[inline(always)]
    pub fn is_t53(&self) -> bool {
        *self == FILTER_TYPE_A::T53
    }
    #[doc = "8 / 4"]
    #[inline(always)]
    pub fn is_t84(&self) -> bool {
        *self == FILTER_TYPE_A::T84
    }
    #[doc = "16 / 8"]
    #[inline(always)]
    pub fn is_t168(&self) -> bool {
        *self == FILTER_TYPE_A::T168
    }
}
#[doc = "Field `filter_type` writer - Filter Type"]
pub type FILTER_TYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FILTER_TYPE_A>;
impl<'a, REG> FILTER_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 / 2"]
    #[inline(always)]
    pub fn t42(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_TYPE_A::T42)
    }
    #[doc = "5 / 3"]
    #[inline(always)]
    pub fn t53(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_TYPE_A::T53)
    }
    #[doc = "8 / 4"]
    #[inline(always)]
    pub fn t84(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_TYPE_A::T84)
    }
    #[doc = "16 / 8"]
    #[inline(always)]
    pub fn t168(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_TYPE_A::T168)
    }
}
#[doc = "Field `filter_en` reader - Filter Enable"]
pub type FILTER_EN_R = crate::BitReader<FILTER_EN_A>;
#[doc = "Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILTER_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
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
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FILTER_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FILTER_EN_A::ENABLE
    }
}
#[doc = "Field `filter_en` writer - Filter Enable"]
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG, FILTER_EN_A>;
impl<'a, REG> FILTER_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FILTER_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Filter Type"]
    #[inline(always)]
    pub fn filter_type(&self) -> FILTER_TYPE_R {
        FILTER_TYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Filter Enable"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Filter Type"]
    #[inline(always)]
    #[must_use]
    pub fn filter_type(&mut self) -> FILTER_TYPE_W<TP_CTRL3_SPEC> {
        FILTER_TYPE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<TP_CTRL3_SPEC> {
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
#[doc = "TP Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tp_ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tp_ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TP_CTRL3_SPEC;
impl crate::RegisterSpec for TP_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tp_ctrl3::R`](R) reader structure"]
impl crate::Readable for TP_CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tp_ctrl3::W`](W) writer structure"]
impl crate::Writable for TP_CTRL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tp_ctrl3 to value 0"]
impl crate::Resettable for TP_CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
