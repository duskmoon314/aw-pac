#[doc = "Register `tv_gint1` reader"]
pub type R = crate::R<TV_GINT1_SPEC>;
#[doc = "Register `tv_gint1` writer"]
pub type W = crate::W<TV_GINT1_SPEC>;
#[doc = "Field `tv_line_int_num` reader - Scan line for TV line trigger(including inactive lines)\n\nSetting it for the specified line for trigger 1.\n\nNote: SY1 is writable only when LINE_TRG1 is disabled."]
pub type TV_LINE_INT_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `tv_line_int_num` writer - Scan line for TV line trigger(including inactive lines)\n\nSetting it for the specified line for trigger 1.\n\nNote: SY1 is writable only when LINE_TRG1 is disabled."]
pub type TV_LINE_INT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Scan line for TV line trigger(including inactive lines)\n\nSetting it for the specified line for trigger 1.\n\nNote: SY1 is writable only when LINE_TRG1 is disabled."]
    #[inline(always)]
    pub fn tv_line_int_num(&self) -> TV_LINE_INT_NUM_R {
        TV_LINE_INT_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Scan line for TV line trigger(including inactive lines)\n\nSetting it for the specified line for trigger 1.\n\nNote: SY1 is writable only when LINE_TRG1 is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tv_line_int_num(&mut self) -> TV_LINE_INT_NUM_W<TV_GINT1_SPEC> {
        TV_LINE_INT_NUM_W::new(self, 0)
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
#[doc = "TV Global Interrupt Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_gint1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_gint1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_GINT1_SPEC;
impl crate::RegisterSpec for TV_GINT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_gint1::R`](R) reader structure"]
impl crate::Readable for TV_GINT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_gint1::W`](W) writer structure"]
impl crate::Writable for TV_GINT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_gint1 to value 0"]
impl crate::Resettable for TV_GINT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
