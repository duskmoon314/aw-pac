#[doc = "Register `lcd_cpu_wr` reader"]
pub type R = crate::R<LCD_CPU_WR_SPEC>;
#[doc = "Register `lcd_cpu_wr` writer"]
pub type W = crate::W<LCD_CPU_WR_SPEC>;
#[doc = "Field `data_wr` writer - Data write on 8080 bus, launch a write operation on 8080 bus."]
pub type DATA_WR_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:23 - Data write on 8080 bus, launch a write operation on 8080 bus."]
    #[inline(always)]
    #[must_use]
    pub fn data_wr(&mut self) -> DATA_WR_W<LCD_CPU_WR_SPEC> {
        DATA_WR_W::new(self, 0)
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
#[doc = "LCD CPU Panel Write Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_wr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_wr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CPU_WR_SPEC;
impl crate::RegisterSpec for LCD_CPU_WR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cpu_wr::R`](R) reader structure"]
impl crate::Readable for LCD_CPU_WR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cpu_wr::W`](W) writer structure"]
impl crate::Writable for LCD_CPU_WR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_wr to value 0"]
impl crate::Resettable for LCD_CPU_WR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
