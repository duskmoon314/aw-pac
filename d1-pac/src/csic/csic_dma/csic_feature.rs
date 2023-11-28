#[doc = "Register `csic_feature` reader"]
pub type R = crate::R<CSIC_FEATURE_SPEC>;
#[doc = "Register `csic_feature` writer"]
pub type W = crate::W<CSIC_FEATURE_SPEC>;
#[doc = "Field `dma0_embedded_fbc` reader - "]
pub type DMA0_EMBEDDED_FBC_R = crate::BitReader<DMA0_EMBEDDED_FBC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_EMBEDDED_FBC_A {
    #[doc = "0: No Embedded DMA"]
    NO_E_MBEDDED = 0,
    #[doc = "1: Embedded FBC"]
    E_MBEDDED = 1,
}
impl From<DMA0_EMBEDDED_FBC_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_EMBEDDED_FBC_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA0_EMBEDDED_FBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA0_EMBEDDED_FBC_A {
        match self.bits {
            false => DMA0_EMBEDDED_FBC_A::NO_E_MBEDDED,
            true => DMA0_EMBEDDED_FBC_A::E_MBEDDED,
        }
    }
    #[doc = "No Embedded DMA"]
    #[inline(always)]
    pub fn is_no_e_mbedded(&self) -> bool {
        *self == DMA0_EMBEDDED_FBC_A::NO_E_MBEDDED
    }
    #[doc = "Embedded FBC"]
    #[inline(always)]
    pub fn is_e_mbedded(&self) -> bool {
        *self == DMA0_EMBEDDED_FBC_A::E_MBEDDED
    }
}
#[doc = "Field `dma0_embedded_lbc` reader - "]
pub type DMA0_EMBEDDED_LBC_R = crate::BitReader<DMA0_EMBEDDED_LBC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_EMBEDDED_LBC_A {
    #[doc = "0: No Embedded LBC"]
    NO_E_MBEDDED = 0,
    #[doc = "1: Embedded LBC"]
    E_MBEDDED = 1,
}
impl From<DMA0_EMBEDDED_LBC_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_EMBEDDED_LBC_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA0_EMBEDDED_LBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA0_EMBEDDED_LBC_A {
        match self.bits {
            false => DMA0_EMBEDDED_LBC_A::NO_E_MBEDDED,
            true => DMA0_EMBEDDED_LBC_A::E_MBEDDED,
        }
    }
    #[doc = "No Embedded LBC"]
    #[inline(always)]
    pub fn is_no_e_mbedded(&self) -> bool {
        *self == DMA0_EMBEDDED_LBC_A::NO_E_MBEDDED
    }
    #[doc = "Embedded LBC"]
    #[inline(always)]
    pub fn is_e_mbedded(&self) -> bool {
        *self == DMA0_EMBEDDED_LBC_A::E_MBEDDED
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma0_embedded_fbc(&self) -> DMA0_EMBEDDED_FBC_R {
        DMA0_EMBEDDED_FBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma0_embedded_lbc(&self) -> DMA0_EMBEDDED_LBC_R {
        DMA0_EMBEDDED_LBC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
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
#[doc = "CSIC DMA Feature List Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_feature::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_feature::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_FEATURE_SPEC;
impl crate::RegisterSpec for CSIC_FEATURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_feature::R`](R) reader structure"]
impl crate::Readable for CSIC_FEATURE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_feature::W`](W) writer structure"]
impl crate::Writable for CSIC_FEATURE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_feature to value 0"]
impl crate::Resettable for CSIC_FEATURE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
