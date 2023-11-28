#[doc = "Register `lcd_frm_tab%s` reader"]
pub type R = crate::R<LCD_FRM_TAB_SPEC>;
#[doc = "Register `lcd_frm_tab%s` writer"]
pub type W = crate::W<LCD_FRM_TAB_SPEC>;
#[doc = "Field `frm_table_value` reader - Set the data used in dither function"]
pub type FRM_TABLE_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `frm_table_value` writer - Set the data used in dither function"]
pub type FRM_TABLE_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set the data used in dither function"]
    #[inline(always)]
    pub fn frm_table_value(&self) -> FRM_TABLE_VALUE_R {
        FRM_TABLE_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the data used in dither function"]
    #[inline(always)]
    #[must_use]
    pub fn frm_table_value(&mut self) -> FRM_TABLE_VALUE_W<LCD_FRM_TAB_SPEC> {
        FRM_TABLE_VALUE_W::new(self, 0)
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
#[doc = "LCD FRM Table Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_frm_tab::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_frm_tab::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_FRM_TAB_SPEC;
impl crate::RegisterSpec for LCD_FRM_TAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_frm_tab::R`](R) reader structure"]
impl crate::Readable for LCD_FRM_TAB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_frm_tab::W`](W) writer structure"]
impl crate::Writable for LCD_FRM_TAB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_frm_tab%s to value 0"]
impl crate::Resettable for LCD_FRM_TAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
