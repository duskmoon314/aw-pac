#[doc = "Register `lcd_basic3` reader"]
pub type R = crate::R<LCD_BASIC3_SPEC>;
#[doc = "Register `lcd_basic3` writer"]
pub type W = crate::W<LCD_BASIC3_SPEC>;
#[doc = "Field `vspw` reader - Tvspw = (VSPW+1) * Thsync\n\nVT/2 > (VSPW+1)"]
pub type VSPW_R = crate::FieldReader<u16>;
#[doc = "Field `vspw` writer - Tvspw = (VSPW+1) * Thsync\n\nVT/2 > (VSPW+1)"]
pub type VSPW_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `hspw` reader - Thspw = (HSPW+1) * Tdclk\n\nHT > (HSPW+1)"]
pub type HSPW_R = crate::FieldReader<u16>;
#[doc = "Field `hspw` writer - Thspw = (HSPW+1) * Tdclk\n\nHT > (HSPW+1)"]
pub type HSPW_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Tvspw = (VSPW+1) * Thsync\n\nVT/2 > (VSPW+1)"]
    #[inline(always)]
    pub fn vspw(&self) -> VSPW_R {
        VSPW_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Thspw = (HSPW+1) * Tdclk\n\nHT > (HSPW+1)"]
    #[inline(always)]
    pub fn hspw(&self) -> HSPW_R {
        HSPW_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Tvspw = (VSPW+1) * Thsync\n\nVT/2 > (VSPW+1)"]
    #[inline(always)]
    #[must_use]
    pub fn vspw(&mut self) -> VSPW_W<LCD_BASIC3_SPEC> {
        VSPW_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Thspw = (HSPW+1) * Tdclk\n\nHT > (HSPW+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hspw(&mut self) -> HSPW_W<LCD_BASIC3_SPEC> {
        HSPW_W::new(self, 16)
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
#[doc = "LCD Basic Timing Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_basic3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_basic3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_BASIC3_SPEC;
impl crate::RegisterSpec for LCD_BASIC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_basic3::R`](R) reader structure"]
impl crate::Readable for LCD_BASIC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_basic3::W`](W) writer structure"]
impl crate::Writable for LCD_BASIC3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_basic3 to value 0"]
impl crate::Resettable for LCD_BASIC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
