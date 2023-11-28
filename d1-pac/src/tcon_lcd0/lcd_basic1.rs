#[doc = "Register `lcd_basic1` reader"]
pub type R = crate::R<LCD_BASIC1_SPEC>;
#[doc = "Register `lcd_basic1` writer"]
pub type W = crate::W<LCD_BASIC1_SPEC>;
#[doc = "Field `hbp` reader - Horizontal back porch (in dclk)\n\nThbp = (HBP +1) * Tdclk"]
pub type HBP_R = crate::FieldReader<u16>;
#[doc = "Field `hbp` writer - Horizontal back porch (in dclk)\n\nThbp = (HBP +1) * Tdclk"]
pub type HBP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ht` reader - Thcycle = (HT+1) * Tdclk\n\nComputation:\n\n1) parallel: HT = X + BLANK\n\nLimitation:\n\n1) parallel: HT >= (HBP +1) + (X+1) +2\n\n2) serial 1: HT >= (HBP +1) + (X+1) *3+2\n\n3) serial 2: HT >= (HBP +1) + (X+1) *3/2+2"]
pub type HT_R = crate::FieldReader<u16>;
#[doc = "Field `ht` writer - Thcycle = (HT+1) * Tdclk\n\nComputation:\n\n1) parallel: HT = X + BLANK\n\nLimitation:\n\n1) parallel: HT >= (HBP +1) + (X+1) +2\n\n2) serial 1: HT >= (HBP +1) + (X+1) *3+2\n\n3) serial 2: HT >= (HBP +1) + (X+1) *3/2+2"]
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal back porch (in dclk)\n\nThbp = (HBP +1) * Tdclk"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:28 - Thcycle = (HT+1) * Tdclk\n\nComputation:\n\n1) parallel: HT = X + BLANK\n\nLimitation:\n\n1) parallel: HT >= (HBP +1) + (X+1) +2\n\n2) serial 1: HT >= (HBP +1) + (X+1) *3+2\n\n3) serial 2: HT >= (HBP +1) + (X+1) *3/2+2"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal back porch (in dclk)\n\nThbp = (HBP +1) * Tdclk"]
    #[inline(always)]
    #[must_use]
    pub fn hbp(&mut self) -> HBP_W<LCD_BASIC1_SPEC> {
        HBP_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Thcycle = (HT+1) * Tdclk\n\nComputation:\n\n1) parallel: HT = X + BLANK\n\nLimitation:\n\n1) parallel: HT >= (HBP +1) + (X+1) +2\n\n2) serial 1: HT >= (HBP +1) + (X+1) *3+2\n\n3) serial 2: HT >= (HBP +1) + (X+1) *3/2+2"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<LCD_BASIC1_SPEC> {
        HT_W::new(self, 16)
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
#[doc = "LCD Basic Timing Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_basic1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_basic1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_BASIC1_SPEC;
impl crate::RegisterSpec for LCD_BASIC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_basic1::R`](R) reader structure"]
impl crate::Readable for LCD_BASIC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_basic1::W`](W) writer structure"]
impl crate::Writable for LCD_BASIC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_basic1 to value 0"]
impl crate::Resettable for LCD_BASIC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
