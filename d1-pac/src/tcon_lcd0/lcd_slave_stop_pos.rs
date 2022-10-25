#[doc = "Register `lcd_slave_stop_pos` reader"]
pub struct R(crate::R<LCD_SLAVE_STOP_POS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_SLAVE_STOP_POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_SLAVE_STOP_POS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_SLAVE_STOP_POS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_slave_stop_pos` writer"]
pub struct W(crate::W<LCD_SLAVE_STOP_POS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_SLAVE_STOP_POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LCD_SLAVE_STOP_POS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_SLAVE_STOP_POS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `stop_val` reader - Set the stop position of the slave LCD. This value is the number of pixels between the stop position and the end of the HFP. Stop_pos = HFP - Stop_val. 0<Stop_pos<HFP-2\n\nNote: Only use in Single DSI mode."]
pub type STOP_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `stop_val` writer - Set the stop position of the slave LCD. This value is the number of pixels between the stop position and the end of the HFP. Stop_pos = HFP - Stop_val. 0<Stop_pos<HFP-2\n\nNote: Only use in Single DSI mode."]
pub type STOP_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_SLAVE_STOP_POS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Set the stop position of the slave LCD. This value is the number of pixels between the stop position and the end of the HFP. Stop_pos = HFP - Stop_val. 0<Stop_pos<HFP-2\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    pub fn stop_val(&self) -> STOP_VAL_R {
        STOP_VAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set the stop position of the slave LCD. This value is the number of pixels between the stop position and the end of the HFP. Stop_pos = HFP - Stop_val. 0<Stop_pos<HFP-2\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    #[must_use]
    pub fn stop_val(&mut self) -> STOP_VAL_W<0> {
        STOP_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Slave Stop Position Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_slave_stop_pos](index.html) module"]
pub struct LCD_SLAVE_STOP_POS_SPEC;
impl crate::RegisterSpec for LCD_SLAVE_STOP_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_slave_stop_pos::R](R) reader structure"]
impl crate::Readable for LCD_SLAVE_STOP_POS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_slave_stop_pos::W](W) writer structure"]
impl crate::Writable for LCD_SLAVE_STOP_POS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_slave_stop_pos to value 0"]
impl crate::Resettable for LCD_SLAVE_STOP_POS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
