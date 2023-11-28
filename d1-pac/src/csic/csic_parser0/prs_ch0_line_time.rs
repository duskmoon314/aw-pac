#[doc = "Register `prs_ch%s0_line_time` reader"]
pub type R = crate::R<PRS_CH0_LINE_TIME_SPEC>;
#[doc = "Field `hsyn_time` reader - Time of H SYNC when vsync is valid\n\nThe unit is csi_top_clk cycle"]
pub type HSYN_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `hblk_time` reader - Time of H Blanking when vsync is valid\n\nThe unit is csi_top_clk cycle"]
pub type HBLK_TIME_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Time of H SYNC when vsync is valid\n\nThe unit is csi_top_clk cycle"]
    #[inline(always)]
    pub fn hsyn_time(&self) -> HSYN_TIME_R {
        HSYN_TIME_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Time of H Blanking when vsync is valid\n\nThe unit is csi_top_clk cycle"]
    #[inline(always)]
    pub fn hblk_time(&self) -> HBLK_TIME_R {
        HBLK_TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Parser Channel\\[i\\] Line Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_ch0_line_time::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_CH0_LINE_TIME_SPEC;
impl crate::RegisterSpec for PRS_CH0_LINE_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs_ch0_line_time::R`](R) reader structure"]
impl crate::Readable for PRS_CH0_LINE_TIME_SPEC {}
#[doc = "`reset()` method sets prs_ch%s0_line_time to value 0"]
impl crate::Resettable for PRS_CH0_LINE_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
