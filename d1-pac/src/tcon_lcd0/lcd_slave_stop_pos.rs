#[doc = "Register `lcd_slave_stop_pos` reader"]
pub type R = crate::R<LCD_SLAVE_STOP_POS_SPEC>;
#[doc = "Register `lcd_slave_stop_pos` writer"]
pub type W = crate::W<LCD_SLAVE_STOP_POS_SPEC>;
#[doc = "Field `stop_val` reader - Set the stop position of the slave LCD. This value is the number of pixels between the stop position and the end of the HFP. Stop_pos = HFP - Stop_val. 0&lt;Stop_pos&lt;HFP-2\n\nNote: Only use in Single DSI mode."]
pub type STOP_VAL_R = crate::FieldReader;
#[doc = "Field `stop_val` writer - Set the stop position of the slave LCD. This value is the number of pixels between the stop position and the end of the HFP. Stop_pos = HFP - Stop_val. 0&lt;Stop_pos&lt;HFP-2\n\nNote: Only use in Single DSI mode."]
pub type STOP_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Set the stop position of the slave LCD. This value is the number of pixels between the stop position and the end of the HFP. Stop_pos = HFP - Stop_val. 0&lt;Stop_pos&lt;HFP-2\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    pub fn stop_val(&self) -> STOP_VAL_R {
        STOP_VAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set the stop position of the slave LCD. This value is the number of pixels between the stop position and the end of the HFP. Stop_pos = HFP - Stop_val. 0&lt;Stop_pos&lt;HFP-2\n\nNote: Only use in Single DSI mode."]
    #[inline(always)]
    #[must_use]
    pub fn stop_val(&mut self) -> STOP_VAL_W<LCD_SLAVE_STOP_POS_SPEC> {
        STOP_VAL_W::new(self, 0)
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
#[doc = "LCD Slave Stop Position Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_slave_stop_pos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_slave_stop_pos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_SLAVE_STOP_POS_SPEC;
impl crate::RegisterSpec for LCD_SLAVE_STOP_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_slave_stop_pos::R`](R) reader structure"]
impl crate::Readable for LCD_SLAVE_STOP_POS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_slave_stop_pos::W`](W) writer structure"]
impl crate::Writable for LCD_SLAVE_STOP_POS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_slave_stop_pos to value 0"]
impl crate::Resettable for LCD_SLAVE_STOP_POS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
