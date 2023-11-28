#[doc = "Register `lcd_basic0` reader"]
pub type R = crate::R<LCD_BASIC0_SPEC>;
#[doc = "Register `lcd_basic0` writer"]
pub type W = crate::W<LCD_BASIC0_SPEC>;
#[doc = "Field `height_y` reader - Panel height is Y+1"]
pub type HEIGHT_Y_R = crate::FieldReader<u16>;
#[doc = "Field `height_y` writer - Panel height is Y+1"]
pub type HEIGHT_Y_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `width_x` reader - Panel width is X+1"]
pub type WIDTH_X_R = crate::FieldReader<u16>;
#[doc = "Field `width_x` writer - Panel width is X+1"]
pub type WIDTH_X_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Panel height is Y+1"]
    #[inline(always)]
    pub fn height_y(&self) -> HEIGHT_Y_R {
        HEIGHT_Y_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Panel width is X+1"]
    #[inline(always)]
    pub fn width_x(&self) -> WIDTH_X_R {
        WIDTH_X_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Panel height is Y+1"]
    #[inline(always)]
    #[must_use]
    pub fn height_y(&mut self) -> HEIGHT_Y_W<LCD_BASIC0_SPEC> {
        HEIGHT_Y_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Panel width is X+1"]
    #[inline(always)]
    #[must_use]
    pub fn width_x(&mut self) -> WIDTH_X_W<LCD_BASIC0_SPEC> {
        WIDTH_X_W::new(self, 16)
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
#[doc = "LCD Basic Timing Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_basic0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_basic0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_BASIC0_SPEC;
impl crate::RegisterSpec for LCD_BASIC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_basic0::R`](R) reader structure"]
impl crate::Readable for LCD_BASIC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_basic0::W`](W) writer structure"]
impl crate::Writable for LCD_BASIC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_basic0 to value 0"]
impl crate::Resettable for LCD_BASIC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
