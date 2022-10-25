#[doc = "Register `lcd_cpu_tri0` reader"]
pub struct R(crate::R<LCD_CPU_TRI0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CPU_TRI0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CPU_TRI0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CPU_TRI0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_cpu_tri0` writer"]
pub struct W(crate::W<LCD_CPU_TRI0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CPU_TRI0_SPEC>;
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
impl From<crate::W<LCD_CPU_TRI0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CPU_TRI0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `block_size` reader - The size of data block. It is usually set as X."]
pub type BLOCK_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `block_size` writer - The size of data block. It is usually set as X."]
pub type BLOCK_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CPU_TRI0_SPEC, u16, u16, 12, O>;
#[doc = "Field `block_space` reader - The spaces between data blocks. It should be set >20*pixel."]
pub type BLOCK_SPACE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `block_space` writer - The spaces between data blocks. It should be set >20*pixel."]
pub type BLOCK_SPACE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CPU_TRI0_SPEC, u16, u16, 12, O>;
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
    pub fn block_size(&mut self) -> BLOCK_SIZE_W<0> {
        BLOCK_SIZE_W::new(self)
    }
    #[doc = "Bits 16:27 - The spaces between data blocks. It should be set >20*pixel."]
    #[inline(always)]
    #[must_use]
    pub fn block_space(&mut self) -> BLOCK_SPACE_W<16> {
        BLOCK_SPACE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD CPU Panel Trigger Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_cpu_tri0](index.html) module"]
pub struct LCD_CPU_TRI0_SPEC;
impl crate::RegisterSpec for LCD_CPU_TRI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_cpu_tri0::R](R) reader structure"]
impl crate::Readable for LCD_CPU_TRI0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_cpu_tri0::W](W) writer structure"]
impl crate::Writable for LCD_CPU_TRI0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_tri0 to value 0"]
impl crate::Resettable for LCD_CPU_TRI0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
