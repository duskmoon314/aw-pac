#[doc = "Register `smhc_csdc` reader"]
pub type R = crate::R<SMHC_CSDC_SPEC>;
#[doc = "Register `smhc_csdc` writer"]
pub type W = crate::W<SMHC_CSDC_SPEC>;
#[doc = "Field `crc_det_para` reader - "]
pub type CRC_DET_PARA_R = crate::FieldReader<CRC_DET_PARA_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRC_DET_PARA_A {
    #[doc = "6: `110`"]
    HS400 = 6,
    #[doc = "3: `11`"]
    OTHER = 3,
}
impl From<CRC_DET_PARA_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_DET_PARA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRC_DET_PARA_A {
    type Ux = u8;
}
impl CRC_DET_PARA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CRC_DET_PARA_A> {
        match self.bits {
            6 => Some(CRC_DET_PARA_A::HS400),
            3 => Some(CRC_DET_PARA_A::OTHER),
            _ => None,
        }
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_hs400(&self) -> bool {
        *self == CRC_DET_PARA_A::HS400
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == CRC_DET_PARA_A::OTHER
    }
}
#[doc = "Field `crc_det_para` writer - "]
pub type CRC_DET_PARA_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CRC_DET_PARA_A>;
impl<'a, REG> CRC_DET_PARA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`110`"]
    #[inline(always)]
    pub fn hs400(self) -> &'a mut crate::W<REG> {
        self.variant(CRC_DET_PARA_A::HS400)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn other(self) -> &'a mut crate::W<REG> {
        self.variant(CRC_DET_PARA_A::OTHER)
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn crc_det_para(&self) -> CRC_DET_PARA_R {
        CRC_DET_PARA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn crc_det_para(&mut self) -> CRC_DET_PARA_W<SMHC_CSDC_SPEC> {
        CRC_DET_PARA_W::new(self, 0)
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
#[doc = "CRC Status Detect Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_csdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_csdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_CSDC_SPEC;
impl crate::RegisterSpec for SMHC_CSDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_csdc::R`](R) reader structure"]
impl crate::Readable for SMHC_CSDC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_csdc::W`](W) writer structure"]
impl crate::Writable for SMHC_CSDC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_csdc to value 0"]
impl crate::Resettable for SMHC_CSDC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
