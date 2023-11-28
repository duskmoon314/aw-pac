#[doc = "Register `csic_ptn_isp_size` reader"]
pub type R = crate::R<CSIC_PTN_ISP_SIZE_SPEC>;
#[doc = "Register `csic_ptn_isp_size` writer"]
pub type W = crate::W<CSIC_PTN_ISP_SIZE_SPEC>;
#[doc = "Field `width` reader - Width Horizontal size, only valid for ISP mode pattern generation."]
pub type WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `width` writer - Width Horizontal size, only valid for ISP mode pattern generation."]
pub type WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `height` reader - Height Vertical size, only valid for ISP mode pattern generation."]
pub type HEIGHT_R = crate::FieldReader<u16>;
#[doc = "Field `height` writer - Height Vertical size, only valid for ISP mode pattern generation."]
pub type HEIGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Width Horizontal size, only valid for ISP mode pattern generation."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Height Vertical size, only valid for ISP mode pattern generation."]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Width Horizontal size, only valid for ISP mode pattern generation."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<CSIC_PTN_ISP_SIZE_SPEC> {
        WIDTH_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Height Vertical size, only valid for ISP mode pattern generation."]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<CSIC_PTN_ISP_SIZE_SPEC> {
        HEIGHT_W::new(self, 16)
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
#[doc = "CSIC Pattern ISP Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_ptn_isp_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_ptn_isp_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_PTN_ISP_SIZE_SPEC;
impl crate::RegisterSpec for CSIC_PTN_ISP_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_ptn_isp_size::R`](R) reader structure"]
impl crate::Readable for CSIC_PTN_ISP_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_ptn_isp_size::W`](W) writer structure"]
impl crate::Writable for CSIC_PTN_ISP_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_ptn_isp_size to value 0"]
impl crate::Resettable for CSIC_PTN_ISP_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
