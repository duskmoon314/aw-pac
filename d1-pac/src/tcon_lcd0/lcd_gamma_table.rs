#[doc = "Register `lcd_gamma_table%s` reader"]
pub type R = crate::R<LCD_GAMMA_TABLE_SPEC>;
#[doc = "Register `lcd_gamma_table%s` writer"]
pub type W = crate::W<LCD_GAMMA_TABLE_SPEC>;
#[doc = "Field `blue_comp` reader - Blue Component"]
pub type BLUE_COMP_R = crate::FieldReader;
#[doc = "Field `blue_comp` writer - Blue Component"]
pub type BLUE_COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `green_comp` reader - Green Component"]
pub type GREEN_COMP_R = crate::FieldReader;
#[doc = "Field `green_comp` writer - Green Component"]
pub type GREEN_COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `red_comp` reader - Red Component"]
pub type RED_COMP_R = crate::FieldReader;
#[doc = "Field `red_comp` writer - Red Component"]
pub type RED_COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Blue Component"]
    #[inline(always)]
    pub fn blue_comp(&self) -> BLUE_COMP_R {
        BLUE_COMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green Component"]
    #[inline(always)]
    pub fn green_comp(&self) -> GREEN_COMP_R {
        GREEN_COMP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Red Component"]
    #[inline(always)]
    pub fn red_comp(&self) -> RED_COMP_R {
        RED_COMP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blue Component"]
    #[inline(always)]
    #[must_use]
    pub fn blue_comp(&mut self) -> BLUE_COMP_W<LCD_GAMMA_TABLE_SPEC> {
        BLUE_COMP_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green Component"]
    #[inline(always)]
    #[must_use]
    pub fn green_comp(&mut self) -> GREEN_COMP_W<LCD_GAMMA_TABLE_SPEC> {
        GREEN_COMP_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Red Component"]
    #[inline(always)]
    #[must_use]
    pub fn red_comp(&mut self) -> RED_COMP_W<LCD_GAMMA_TABLE_SPEC> {
        RED_COMP_W::new(self, 16)
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
#[doc = "LCD Gamma Table Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_gamma_table::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_gamma_table::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_GAMMA_TABLE_SPEC;
impl crate::RegisterSpec for LCD_GAMMA_TABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_gamma_table::R`](R) reader structure"]
impl crate::Readable for LCD_GAMMA_TABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_gamma_table::W`](W) writer structure"]
impl crate::Writable for LCD_GAMMA_TABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_gamma_table%s to value 0"]
impl crate::Resettable for LCD_GAMMA_TABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
