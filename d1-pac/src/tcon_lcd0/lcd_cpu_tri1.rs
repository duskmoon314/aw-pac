#[doc = "Register `lcd_cpu_tri1` reader"]
pub type R = crate::R<LCD_CPU_TRI1_SPEC>;
#[doc = "Register `lcd_cpu_tri1` writer"]
pub type W = crate::W<LCD_CPU_TRI1_SPEC>;
#[doc = "Field `block_num` reader - The number of data blocks. It is usually set as Y."]
pub type BLOCK_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `block_num` writer - The number of data blocks. It is usually set as Y."]
pub type BLOCK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `block_current_num` reader - Shows the current data block transmitting to panel."]
pub type BLOCK_CURRENT_NUM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The number of data blocks. It is usually set as Y."]
    #[inline(always)]
    pub fn block_num(&self) -> BLOCK_NUM_R {
        BLOCK_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Shows the current data block transmitting to panel."]
    #[inline(always)]
    pub fn block_current_num(&self) -> BLOCK_CURRENT_NUM_R {
        BLOCK_CURRENT_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The number of data blocks. It is usually set as Y."]
    #[inline(always)]
    #[must_use]
    pub fn block_num(&mut self) -> BLOCK_NUM_W<LCD_CPU_TRI1_SPEC> {
        BLOCK_NUM_W::new(self, 0)
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
#[doc = "LCD CPU Panel Trigger Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CPU_TRI1_SPEC;
impl crate::RegisterSpec for LCD_CPU_TRI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cpu_tri1::R`](R) reader structure"]
impl crate::Readable for LCD_CPU_TRI1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cpu_tri1::W`](W) writer structure"]
impl crate::Writable for LCD_CPU_TRI1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_tri1 to value 0"]
impl crate::Resettable for LCD_CPU_TRI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
