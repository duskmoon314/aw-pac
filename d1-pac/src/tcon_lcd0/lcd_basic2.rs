#[doc = "Register `lcd_basic2` reader"]
pub type R = crate::R<LCD_BASIC2_SPEC>;
#[doc = "Register `lcd_basic2` writer"]
pub type W = crate::W<LCD_BASIC2_SPEC>;
#[doc = "Field `vbp` reader - Tvbp = (VBP +1) * Thsync"]
pub type VBP_R = crate::FieldReader<u16>;
#[doc = "Field `vbp` writer - Tvbp = (VBP +1) * Thsync"]
pub type VBP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `vt` reader - TVT = (VT)/2 * Thsync\n\nVT/2 >= (VBP+1 ) + (Y+1) +2"]
pub type VT_R = crate::FieldReader<u16>;
#[doc = "Field `vt` writer - TVT = (VT)/2 * Thsync\n\nVT/2 >= (VBP+1 ) + (Y+1) +2"]
pub type VT_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:11 - Tvbp = (VBP +1) * Thsync"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:28 - TVT = (VT)/2 * Thsync\n\nVT/2 >= (VBP+1 ) + (Y+1) +2"]
    #[inline(always)]
    pub fn vt(&self) -> VT_R {
        VT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Tvbp = (VBP +1) * Thsync"]
    #[inline(always)]
    #[must_use]
    pub fn vbp(&mut self) -> VBP_W<LCD_BASIC2_SPEC> {
        VBP_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - TVT = (VT)/2 * Thsync\n\nVT/2 >= (VBP+1 ) + (Y+1) +2"]
    #[inline(always)]
    #[must_use]
    pub fn vt(&mut self) -> VT_W<LCD_BASIC2_SPEC> {
        VT_W::new(self, 16)
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
#[doc = "LCD Basic Timing Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_basic2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_basic2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_BASIC2_SPEC;
impl crate::RegisterSpec for LCD_BASIC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_basic2::R`](R) reader structure"]
impl crate::Readable for LCD_BASIC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_basic2::W`](W) writer structure"]
impl crate::Writable for LCD_BASIC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_basic2 to value 0"]
impl crate::Resettable for LCD_BASIC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
