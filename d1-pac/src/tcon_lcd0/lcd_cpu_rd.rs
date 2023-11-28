#[doc = "Register `lcd_cpu_rd%s` reader"]
pub type R = crate::R<LCD_CPU_RD_SPEC>;
#[doc = "Register `lcd_cpu_rd%s` writer"]
pub type W = crate::W<LCD_CPU_RD_SPEC>;
#[doc = "Field `data_rd0` reader - Data read on 8080 bus, launch a new read operation on 8080 bus."]
pub type DATA_RD0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Data read on 8080 bus, launch a new read operation on 8080 bus."]
    #[inline(always)]
    pub fn data_rd0(&self) -> DATA_RD0_R {
        DATA_RD0_R::new(self.bits & 0x00ff_ffff)
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
#[doc = "LCD CPU Panel Read Data Register\\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_rd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_rd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CPU_RD_SPEC;
impl crate::RegisterSpec for LCD_CPU_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cpu_rd::R`](R) reader structure"]
impl crate::Readable for LCD_CPU_RD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cpu_rd::W`](W) writer structure"]
impl crate::Writable for LCD_CPU_RD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_rd%s to value 0"]
impl crate::Resettable for LCD_CPU_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
