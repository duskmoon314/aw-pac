#[doc = "Register `smhc_ctype` reader"]
pub type R = crate::R<SMHC_CTYPE_SPEC>;
#[doc = "Register `smhc_ctype` writer"]
pub type W = crate::W<SMHC_CTYPE_SPEC>;
#[doc = "Field `card_wid` reader - Card Width"]
pub type CARD_WID_R = crate::FieldReader<CARD_WID_A>;
#[doc = "Card Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CARD_WID_A {
    #[doc = "0: 1-bit width"]
    B1 = 0,
    #[doc = "1: 4-bit width"]
    B4 = 1,
    #[doc = "2: 8-bit width"]
    B8 = 2,
}
impl From<CARD_WID_A> for u8 {
    #[inline(always)]
    fn from(variant: CARD_WID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CARD_WID_A {
    type Ux = u8;
}
impl CARD_WID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CARD_WID_A> {
        match self.bits {
            0 => Some(CARD_WID_A::B1),
            1 => Some(CARD_WID_A::B4),
            2 => Some(CARD_WID_A::B8),
            _ => None,
        }
    }
    #[doc = "1-bit width"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CARD_WID_A::B1
    }
    #[doc = "4-bit width"]
    #[inline(always)]
    pub fn is_b4(&self) -> bool {
        *self == CARD_WID_A::B4
    }
    #[doc = "8-bit width"]
    #[inline(always)]
    pub fn is_b8(&self) -> bool {
        *self == CARD_WID_A::B8
    }
}
#[doc = "Field `card_wid` writer - Card Width"]
pub type CARD_WID_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CARD_WID_A>;
impl<'a, REG> CARD_WID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1-bit width"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_WID_A::B1)
    }
    #[doc = "4-bit width"]
    #[inline(always)]
    pub fn b4(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_WID_A::B4)
    }
    #[doc = "8-bit width"]
    #[inline(always)]
    pub fn b8(self) -> &'a mut crate::W<REG> {
        self.variant(CARD_WID_A::B8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Card Width"]
    #[inline(always)]
    pub fn card_wid(&self) -> CARD_WID_R {
        CARD_WID_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Card Width"]
    #[inline(always)]
    #[must_use]
    pub fn card_wid(&mut self) -> CARD_WID_W<SMHC_CTYPE_SPEC> {
        CARD_WID_W::new(self, 0)
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
#[doc = "Bus Width Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ctype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_ctype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_CTYPE_SPEC;
impl crate::RegisterSpec for SMHC_CTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_ctype::R`](R) reader structure"]
impl crate::Readable for SMHC_CTYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_ctype::W`](W) writer structure"]
impl crate::Writable for SMHC_CTYPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_ctype to value 0"]
impl crate::Resettable for SMHC_CTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
