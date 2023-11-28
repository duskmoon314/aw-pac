#[doc = "Register `lcd_ceu_coef_add%s` reader"]
pub type R = crate::R<LCD_CEU_COEF_ADD_SPEC>;
#[doc = "Register `lcd_ceu_coef_add%s` writer"]
pub type W = crate::W<LCD_CEU_COEF_ADD_SPEC>;
#[doc = "Field `ceu_coef_add_value` reader - Signed 19-bit value, range of (-16384, 16384)."]
pub type CEU_COEF_ADD_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `ceu_coef_add_value` writer - Signed 19-bit value, range of (-16384, 16384)."]
pub type CEU_COEF_ADD_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - Signed 19-bit value, range of (-16384, 16384)."]
    #[inline(always)]
    pub fn ceu_coef_add_value(&self) -> CEU_COEF_ADD_VALUE_R {
        CEU_COEF_ADD_VALUE_R::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - Signed 19-bit value, range of (-16384, 16384)."]
    #[inline(always)]
    #[must_use]
    pub fn ceu_coef_add_value(&mut self) -> CEU_COEF_ADD_VALUE_W<LCD_CEU_COEF_ADD_SPEC> {
        CEU_COEF_ADD_VALUE_W::new(self, 0)
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
#[doc = "LCD CEU Coefficient Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ceu_coef_add::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ceu_coef_add::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CEU_COEF_ADD_SPEC;
impl crate::RegisterSpec for LCD_CEU_COEF_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ceu_coef_add::R`](R) reader structure"]
impl crate::Readable for LCD_CEU_COEF_ADD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_ceu_coef_add::W`](W) writer structure"]
impl crate::Writable for LCD_CEU_COEF_ADD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_ceu_coef_add%s to value 0"]
impl crate::Resettable for LCD_CEU_COEF_ADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
