#[doc = "Register `csic_bist_data_mask` reader"]
pub type R = crate::R<CSIC_BIST_DATA_MASK_SPEC>;
#[doc = "Register `csic_bist_data_mask` writer"]
pub type W = crate::W<CSIC_BIST_DATA_MASK_SPEC>;
#[doc = "Field `bist_data_mask` reader - BIST data mask"]
pub type BIST_DATA_MASK_R = crate::FieldReader<BIST_DATA_MASK_A>;
#[doc = "BIST data mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum BIST_DATA_MASK_A {
    #[doc = "0: `0`"]
    UNMASK = 0,
    #[doc = "1: `1`"]
    MASK = 1,
}
impl From<BIST_DATA_MASK_A> for u32 {
    #[inline(always)]
    fn from(variant: BIST_DATA_MASK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BIST_DATA_MASK_A {
    type Ux = u32;
}
impl BIST_DATA_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BIST_DATA_MASK_A> {
        match self.bits {
            0 => Some(BIST_DATA_MASK_A::UNMASK),
            1 => Some(BIST_DATA_MASK_A::MASK),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        *self == BIST_DATA_MASK_A::UNMASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == BIST_DATA_MASK_A::MASK
    }
}
#[doc = "Field `bist_data_mask` writer - BIST data mask"]
pub type BIST_DATA_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, BIST_DATA_MASK_A>;
impl<'a, REG> BIST_DATA_MASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_DATA_MASK_A::UNMASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_DATA_MASK_A::MASK)
    }
}
impl R {
    #[doc = "Bits 0:31 - BIST data mask"]
    #[inline(always)]
    pub fn bist_data_mask(&self) -> BIST_DATA_MASK_R {
        BIST_DATA_MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BIST data mask"]
    #[inline(always)]
    #[must_use]
    pub fn bist_data_mask(&mut self) -> BIST_DATA_MASK_W<CSIC_BIST_DATA_MASK_SPEC> {
        BIST_DATA_MASK_W::new(self, 0)
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
#[doc = "CSIC BIST Data Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_bist_data_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_bist_data_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_BIST_DATA_MASK_SPEC;
impl crate::RegisterSpec for CSIC_BIST_DATA_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_bist_data_mask::R`](R) reader structure"]
impl crate::Readable for CSIC_BIST_DATA_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_bist_data_mask::W`](W) writer structure"]
impl crate::Writable for CSIC_BIST_DATA_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_bist_data_mask to value 0"]
impl crate::Resettable for CSIC_BIST_DATA_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
