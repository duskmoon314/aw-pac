#[doc = "Register `hc_ls_threshold` reader"]
pub type R = crate::R<HC_LS_THRESHOLD_SPEC>;
#[doc = "Register `hc_ls_threshold` writer"]
pub type W = crate::W<HC_LS_THRESHOLD_SPEC>;
#[doc = "Field `ls_threshold` reader - LSThreshold\n\nThis field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction. The transaction is started only if FrameRemaining this field. The value is calculated by HCD with the consideration of transmission and setup overhead."]
pub type LS_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `ls_threshold` writer - LSThreshold\n\nThis field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction. The transaction is started only if FrameRemaining this field. The value is calculated by HCD with the consideration of transmission and setup overhead."]
pub type LS_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - LSThreshold\n\nThis field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction. The transaction is started only if FrameRemaining this field. The value is calculated by HCD with the consideration of transmission and setup overhead."]
    #[inline(always)]
    pub fn ls_threshold(&self) -> LS_THRESHOLD_R {
        LS_THRESHOLD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - LSThreshold\n\nThis field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction. The transaction is started only if FrameRemaining this field. The value is calculated by HCD with the consideration of transmission and setup overhead."]
    #[inline(always)]
    #[must_use]
    pub fn ls_threshold(&mut self) -> LS_THRESHOLD_W<HC_LS_THRESHOLD_SPEC> {
        LS_THRESHOLD_W::new(self, 0)
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
#[doc = "OHCI LS Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_ls_threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_ls_threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_LS_THRESHOLD_SPEC;
impl crate::RegisterSpec for HC_LS_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_ls_threshold::R`](R) reader structure"]
impl crate::Readable for HC_LS_THRESHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_ls_threshold::W`](W) writer structure"]
impl crate::Writable for HC_LS_THRESHOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_ls_threshold to value 0x0628"]
impl crate::Resettable for HC_LS_THRESHOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0628;
}
