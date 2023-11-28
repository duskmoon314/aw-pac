#[doc = "Register `lcd_debug` reader"]
pub type R = crate::R<LCD_DEBUG_SPEC>;
#[doc = "Register `lcd_debug` writer"]
pub type W = crate::W<LCD_DEBUG_SPEC>;
#[doc = "Field `lcd_current_line` reader - The current scan line"]
pub type LCD_CURRENT_LINE_R = crate::FieldReader<u16>;
#[doc = "Field `lcd_field_pol` reader - The flag indicates the current field polarity"]
pub type LCD_FIELD_POL_R = crate::BitReader<LCD_FIELD_POL_A>;
#[doc = "The flag indicates the current field polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_FIELD_POL_A {
    #[doc = "0: Second field"]
    SECOND = 0,
    #[doc = "1: First field"]
    FIRST = 1,
}
impl From<LCD_FIELD_POL_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_FIELD_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_FIELD_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCD_FIELD_POL_A {
        match self.bits {
            false => LCD_FIELD_POL_A::SECOND,
            true => LCD_FIELD_POL_A::FIRST,
        }
    }
    #[doc = "Second field"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == LCD_FIELD_POL_A::SECOND
    }
    #[doc = "First field"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == LCD_FIELD_POL_A::FIRST
    }
}
#[doc = "Field `lcd_fifo_underflow` reader - The flag shows whether the fifos in underflow status"]
pub type LCD_FIFO_UNDERFLOW_R = crate::BitReader<LCD_FIFO_UNDERFLOW_A>;
#[doc = "The flag shows whether the fifos in underflow status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD_FIFO_UNDERFLOW_A {
    #[doc = "0: Not underflow"]
    NOT_UNDERFLOW = 0,
    #[doc = "1: Underflow"]
    U_NDERFLOW = 1,
}
impl From<LCD_FIFO_UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_FIFO_UNDERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD_FIFO_UNDERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCD_FIFO_UNDERFLOW_A {
        match self.bits {
            false => LCD_FIFO_UNDERFLOW_A::NOT_UNDERFLOW,
            true => LCD_FIFO_UNDERFLOW_A::U_NDERFLOW,
        }
    }
    #[doc = "Not underflow"]
    #[inline(always)]
    pub fn is_not_underflow(&self) -> bool {
        *self == LCD_FIFO_UNDERFLOW_A::NOT_UNDERFLOW
    }
    #[doc = "Underflow"]
    #[inline(always)]
    pub fn is_u_nderflow(&self) -> bool {
        *self == LCD_FIFO_UNDERFLOW_A::U_NDERFLOW
    }
}
impl R {
    #[doc = "Bits 16:27 - The current scan line"]
    #[inline(always)]
    pub fn lcd_current_line(&self) -> LCD_CURRENT_LINE_R {
        LCD_CURRENT_LINE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 29 - The flag indicates the current field polarity"]
    #[inline(always)]
    pub fn lcd_field_pol(&self) -> LCD_FIELD_POL_R {
        LCD_FIELD_POL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - The flag shows whether the fifos in underflow status"]
    #[inline(always)]
    pub fn lcd_fifo_underflow(&self) -> LCD_FIFO_UNDERFLOW_R {
        LCD_FIFO_UNDERFLOW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
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
#[doc = "LCD Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_debug::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_debug::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_DEBUG_SPEC;
impl crate::RegisterSpec for LCD_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_debug::R`](R) reader structure"]
impl crate::Readable for LCD_DEBUG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_debug::W`](W) writer structure"]
impl crate::Writable for LCD_DEBUG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_debug to value 0"]
impl crate::Resettable for LCD_DEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
