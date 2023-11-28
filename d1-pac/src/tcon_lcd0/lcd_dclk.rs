#[doc = "Register `lcd_dclk` reader"]
pub type R = crate::R<LCD_DCLK_SPEC>;
#[doc = "Register `lcd_dclk` writer"]
pub type W = crate::W<LCD_DCLK_SPEC>;
#[doc = "Field `lcd_dclk_div` reader - Tdclk = Tsclk/DCLKDIV\n\nNote:\n\n1.If dclk1 and dclk2 are used, DCLKDIV >=6\n\n2.If only dclk is used, DCLKDIV >=1"]
pub type LCD_DCLK_DIV_R = crate::FieldReader;
#[doc = "Field `lcd_dclk_div` writer - Tdclk = Tsclk/DCLKDIV\n\nNote:\n\n1.If dclk1 and dclk2 are used, DCLKDIV >=6\n\n2.If only dclk is used, DCLKDIV >=1"]
pub type LCD_DCLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `lcd_dclk_en` reader - LCD clock enable\n\n"]
pub type LCD_DCLK_EN_R = crate::FieldReader;
#[doc = "Field `lcd_dclk_en` writer - LCD clock enable\n\n"]
pub type LCD_DCLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - Tdclk = Tsclk/DCLKDIV\n\nNote:\n\n1.If dclk1 and dclk2 are used, DCLKDIV >=6\n\n2.If only dclk is used, DCLKDIV >=1"]
    #[inline(always)]
    pub fn lcd_dclk_div(&self) -> LCD_DCLK_DIV_R {
        LCD_DCLK_DIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 28:31 - LCD clock enable\n\n"]
    #[inline(always)]
    pub fn lcd_dclk_en(&self) -> LCD_DCLK_EN_R {
        LCD_DCLK_EN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Tdclk = Tsclk/DCLKDIV\n\nNote:\n\n1.If dclk1 and dclk2 are used, DCLKDIV >=6\n\n2.If only dclk is used, DCLKDIV >=1"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dclk_div(&mut self) -> LCD_DCLK_DIV_W<LCD_DCLK_SPEC> {
        LCD_DCLK_DIV_W::new(self, 0)
    }
    #[doc = "Bits 28:31 - LCD clock enable\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dclk_en(&mut self) -> LCD_DCLK_EN_W<LCD_DCLK_SPEC> {
        LCD_DCLK_EN_W::new(self, 28)
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
#[doc = "LCD Data Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_dclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_dclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_DCLK_SPEC;
impl crate::RegisterSpec for LCD_DCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_dclk::R`](R) reader structure"]
impl crate::Readable for LCD_DCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_dclk::W`](W) writer structure"]
impl crate::Writable for LCD_DCLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_dclk to value 0"]
impl crate::Resettable for LCD_DCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
