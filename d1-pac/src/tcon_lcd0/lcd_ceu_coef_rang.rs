#[doc = "Register `lcd_ceu_coef_rang%s` reader"]
pub type R = crate::R<LCD_CEU_COEF_RANG_SPEC>;
#[doc = "Register `lcd_ceu_coef_rang%s` writer"]
pub type W = crate::W<LCD_CEU_COEF_RANG_SPEC>;
#[doc = "Field `ceu_coef_range_max` reader - Unsigned 8-bit value, range of \\[0, 255\\]."]
pub type CEU_COEF_RANGE_MAX_R = crate::FieldReader;
#[doc = "Field `ceu_coef_range_max` writer - Unsigned 8-bit value, range of \\[0, 255\\]."]
pub type CEU_COEF_RANGE_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ceu_coef_range_min` reader - Unsigned 8-bit value, range of \\[0, 255\\]."]
pub type CEU_COEF_RANGE_MIN_R = crate::FieldReader;
#[doc = "Field `ceu_coef_range_min` writer - Unsigned 8-bit value, range of \\[0, 255\\]."]
pub type CEU_COEF_RANGE_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Unsigned 8-bit value, range of \\[0, 255\\]."]
    #[inline(always)]
    pub fn ceu_coef_range_max(&self) -> CEU_COEF_RANGE_MAX_R {
        CEU_COEF_RANGE_MAX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unsigned 8-bit value, range of \\[0, 255\\]."]
    #[inline(always)]
    pub fn ceu_coef_range_min(&self) -> CEU_COEF_RANGE_MIN_R {
        CEU_COEF_RANGE_MIN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Unsigned 8-bit value, range of \\[0, 255\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ceu_coef_range_max(&mut self) -> CEU_COEF_RANGE_MAX_W<LCD_CEU_COEF_RANG_SPEC> {
        CEU_COEF_RANGE_MAX_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Unsigned 8-bit value, range of \\[0, 255\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ceu_coef_range_min(&mut self) -> CEU_COEF_RANGE_MIN_W<LCD_CEU_COEF_RANG_SPEC> {
        CEU_COEF_RANGE_MIN_W::new(self, 16)
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
#[doc = "LCD CEU Coefficient Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ceu_coef_rang::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ceu_coef_rang::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CEU_COEF_RANG_SPEC;
impl crate::RegisterSpec for LCD_CEU_COEF_RANG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ceu_coef_rang::R`](R) reader structure"]
impl crate::Readable for LCD_CEU_COEF_RANG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_ceu_coef_rang::W`](W) writer structure"]
impl crate::Writable for LCD_CEU_COEF_RANG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_ceu_coef_rang%s to value 0"]
impl crate::Resettable for LCD_CEU_COEF_RANG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
