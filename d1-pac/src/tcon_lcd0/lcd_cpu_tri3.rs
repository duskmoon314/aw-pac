#[doc = "Register `lcd_cpu_tri3` reader"]
pub struct R(crate::R<LCD_CPU_TRI3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CPU_TRI3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CPU_TRI3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CPU_TRI3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_cpu_tri3` writer"]
pub struct W(crate::W<LCD_CPU_TRI3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CPU_TRI3_SPEC>;
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
impl From<crate::W<LCD_CPU_TRI3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CPU_TRI3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `counter_m` reader - The value of counter factor"]
pub type COUNTER_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `counter_m` writer - The value of counter factor"]
pub type COUNTER_M_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CPU_TRI3_SPEC, u8, u8, 8, O>;
#[doc = "Field `counter_n` reader - The value of counter factor"]
pub type COUNTER_N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `counter_n` writer - The value of counter factor"]
pub type COUNTER_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CPU_TRI3_SPEC, u16, u16, 16, O>;
#[doc = "Field `tri_int_mode` reader - When set as 01, the Tri_Counter_Int occurs in cycle of (Count_N+1) * (Count_M+1) * 4 dclk.\n\nWhen set as 10 or 11, the io0 is map as TE input."]
pub type TRI_INT_MODE_R = crate::FieldReader<u8, TRI_INT_MODE_A>;
#[doc = "When set as 01, the Tri_Counter_Int occurs in cycle of (Count_N+1) * (Count_M+1) * 4 dclk.\n\nWhen set as 10 or 11, the io0 is map as TE input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRI_INT_MODE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Counter mode"]
    C_OUNTER = 1,
    #[doc = "2: TE rising mode"]
    TE_RISING = 2,
    #[doc = "3: TE falling mode"]
    TE_FALLING = 3,
}
impl From<TRI_INT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRI_INT_MODE_A) -> Self {
        variant as _
    }
}
impl TRI_INT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRI_INT_MODE_A {
        match self.bits {
            0 => TRI_INT_MODE_A::DISABLE,
            1 => TRI_INT_MODE_A::C_OUNTER,
            2 => TRI_INT_MODE_A::TE_RISING,
            3 => TRI_INT_MODE_A::TE_FALLING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TRI_INT_MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `C_OUNTER`"]
    #[inline(always)]
    pub fn is_c_ounter(&self) -> bool {
        *self == TRI_INT_MODE_A::C_OUNTER
    }
    #[doc = "Checks if the value of the field is `TE_RISING`"]
    #[inline(always)]
    pub fn is_te_rising(&self) -> bool {
        *self == TRI_INT_MODE_A::TE_RISING
    }
    #[doc = "Checks if the value of the field is `TE_FALLING`"]
    #[inline(always)]
    pub fn is_te_falling(&self) -> bool {
        *self == TRI_INT_MODE_A::TE_FALLING
    }
}
#[doc = "Field `tri_int_mode` writer - When set as 01, the Tri_Counter_Int occurs in cycle of (Count_N+1) * (Count_M+1) * 4 dclk.\n\nWhen set as 10 or 11, the io0 is map as TE input."]
pub type TRI_INT_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCD_CPU_TRI3_SPEC, u8, TRI_INT_MODE_A, 2, O>;
impl<'a, const O: u8> TRI_INT_MODE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TRI_INT_MODE_A::DISABLE)
    }
    #[doc = "Counter mode"]
    #[inline(always)]
    pub fn c_ounter(self) -> &'a mut W {
        self.variant(TRI_INT_MODE_A::C_OUNTER)
    }
    #[doc = "TE rising mode"]
    #[inline(always)]
    pub fn te_rising(self) -> &'a mut W {
        self.variant(TRI_INT_MODE_A::TE_RISING)
    }
    #[doc = "TE falling mode"]
    #[inline(always)]
    pub fn te_falling(self) -> &'a mut W {
        self.variant(TRI_INT_MODE_A::TE_FALLING)
    }
}
impl R {
    #[doc = "Bits 0:7 - The value of counter factor"]
    #[inline(always)]
    pub fn counter_m(&self) -> COUNTER_M_R {
        COUNTER_M_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - The value of counter factor"]
    #[inline(always)]
    pub fn counter_n(&self) -> COUNTER_N_R {
        COUNTER_N_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 28:29 - When set as 01, the Tri_Counter_Int occurs in cycle of (Count_N+1) * (Count_M+1) * 4 dclk.\n\nWhen set as 10 or 11, the io0 is map as TE input."]
    #[inline(always)]
    pub fn tri_int_mode(&self) -> TRI_INT_MODE_R {
        TRI_INT_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The value of counter factor"]
    #[inline(always)]
    #[must_use]
    pub fn counter_m(&mut self) -> COUNTER_M_W<0> {
        COUNTER_M_W::new(self)
    }
    #[doc = "Bits 8:23 - The value of counter factor"]
    #[inline(always)]
    #[must_use]
    pub fn counter_n(&mut self) -> COUNTER_N_W<8> {
        COUNTER_N_W::new(self)
    }
    #[doc = "Bits 28:29 - When set as 01, the Tri_Counter_Int occurs in cycle of (Count_N+1) * (Count_M+1) * 4 dclk.\n\nWhen set as 10 or 11, the io0 is map as TE input."]
    #[inline(always)]
    #[must_use]
    pub fn tri_int_mode(&mut self) -> TRI_INT_MODE_W<28> {
        TRI_INT_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD CPU Panel Trigger Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_cpu_tri3](index.html) module"]
pub struct LCD_CPU_TRI3_SPEC;
impl crate::RegisterSpec for LCD_CPU_TRI3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_cpu_tri3::R](R) reader structure"]
impl crate::Readable for LCD_CPU_TRI3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_cpu_tri3::W](W) writer structure"]
impl crate::Writable for LCD_CPU_TRI3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cpu_tri3 to value 0"]
impl crate::Resettable for LCD_CPU_TRI3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
