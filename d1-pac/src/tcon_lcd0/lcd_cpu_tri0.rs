#[doc = "Register `lcd_cpu_tri0` reader"]
pub type R = crate::R<LCD_CPU_TRI0_SPEC>;
#[doc = "Register `lcd_cpu_tri0` writer"]
pub type W = crate::W<LCD_CPU_TRI0_SPEC>;
#[doc = "Field `block_size` reader - The size of data block. It is usually set as X."]
pub type BLOCK_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `block_size` writer - The size of data block. It is usually set as X."]
pub type BLOCK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `block_space` reader - The spaces between data blocks. It should be set >20*pixel."]
pub type BLOCK_SPACE_R = crate::FieldReader<u16>;
#[doc = "Field `block_space` writer - The spaces between data blocks. It should be set >20*pixel."]
pub type BLOCK_SPACE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - The size of data block. It is usually set as X."]
    #[inline(always)]
    pub fn block_size(&self) -> BLOCK_SIZE_R {
        BLOCK_SIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - The spaces between data blocks. It should be set >20*pixel."]
    #[inline(always)]
    pub fn block_space(&self) -> BLOCK_SPACE_R {
        BLOCK_SPACE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The size of data block. It is usually set as X."]
    #[inline(always)]
    #[must_use]
    pub fn block_size(&mut self) -> BLOCK_SIZE_W<LCD_CPU_TRI0_SPEC> {
        BLOCK_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - The spaces between data blocks. It should be set >20*pixel."]
    #[inline(always)]
    #[must_use]
    pub fn block_space(&mut self) -> BLOCK_SPACE_W<LCD_CPU_TRI0_SPEC> {
        BLOCK_SPACE_W::new(self, 16)
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
#[doc = "LCD CPU Panel Trigger Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cpu_tri0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cpu_tri0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CPU_TRI0_SPEC;
impl crate::RegisterSpec for LCD_CPU_TRI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cpu_tri0::R`](R) reader structure"]
impl crate::Readable for LCD_CPU_TRI0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cpu_tri0::W`](W) writer structure"]
impl crate::Writable for LCD_CPU_TRI0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_tri0 to value 0"]
impl crate::Resettable for LCD_CPU_TRI0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
