#[doc = "Register `lcd_ceu_coef_mul%s` reader"]
pub type R = crate::R<LCD_CEU_COEF_MUL_SPEC>;
#[doc = "Register `lcd_ceu_coef_mul%s` writer"]
pub type W = crate::W<LCD_CEU_COEF_MUL_SPEC>;
#[doc = "Field `ceu_coef_mul_value` reader - Signed 13-bit value, range of (-16,16)."]
pub type CEU_COEF_MUL_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `ceu_coef_mul_value` writer - Signed 13-bit value, range of (-16,16)."]
pub type CEU_COEF_MUL_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Signed 13-bit value, range of (-16,16)."]
    #[inline(always)]
    pub fn ceu_coef_mul_value(&self) -> CEU_COEF_MUL_VALUE_R {
        CEU_COEF_MUL_VALUE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Signed 13-bit value, range of (-16,16)."]
    #[inline(always)]
    #[must_use]
    pub fn ceu_coef_mul_value(&mut self) -> CEU_COEF_MUL_VALUE_W<LCD_CEU_COEF_MUL_SPEC> {
        CEU_COEF_MUL_VALUE_W::new(self, 0)
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
#[doc = "LCD CEU Coefficient Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ceu_coef_mul::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ceu_coef_mul::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CEU_COEF_MUL_SPEC;
impl crate::RegisterSpec for LCD_CEU_COEF_MUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ceu_coef_mul::R`](R) reader structure"]
impl crate::Readable for LCD_CEU_COEF_MUL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_ceu_coef_mul::W`](W) writer structure"]
impl crate::Writable for LCD_CEU_COEF_MUL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_ceu_coef_mul%s to value 0"]
impl crate::Resettable for LCD_CEU_COEF_MUL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
