#[doc = "Register `tp_ctrl0` reader"]
pub struct R(crate::R<TP_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tp_ctrl0` writer"]
pub struct W(crate::W<TP_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_CTRL0_SPEC>;
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
impl From<crate::W<TP_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tacq` reader - Touch panel ADC acquire time"]
pub type TACQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tacq` writer - Touch panel ADC acquire time"]
pub type TACQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TP_CTRL0_SPEC, u16, u16, 16, O>;
#[doc = "Field `fs_div` reader - ADC Sample Frequency Divider"]
pub type FS_DIV_R = crate::FieldReader<u8, FS_DIV_A>;
#[doc = "ADC Sample Frequency Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FS_DIV_A {
    #[doc = "0: CLK_IN / 2 ^ (20 - 0)"]
    C2P0 = 0,
    #[doc = "1: CLK_IN / 2 ^ (20 - 1)"]
    C2P1 = 1,
    #[doc = "2: CLK_IN / 2 ^ (20 - 2)"]
    C2P2 = 2,
    #[doc = "3: CLK_IN / 2 ^ (20 - 3)"]
    C2P3 = 3,
    #[doc = "4: CLK_IN / 2 ^ (20 - 4)"]
    C2P4 = 4,
    #[doc = "5: CLK_IN / 2 ^ (20 - 5)"]
    C2P5 = 5,
    #[doc = "6: CLK_IN / 2 ^ (20 - 6)"]
    C2P6 = 6,
    #[doc = "7: CLK_IN / 2 ^ (20 - 7)"]
    C2P7 = 7,
    #[doc = "8: CLK_IN / 2 ^ (20 - 8)"]
    C2P8 = 8,
    #[doc = "9: CLK_IN / 2 ^ (20 - 9)"]
    C2P9 = 9,
    #[doc = "10: CLK_IN / 2 ^ (20 - 10)"]
    C2P10 = 10,
    #[doc = "11: CLK_IN / 2 ^ (20 - 11)"]
    C2P11 = 11,
    #[doc = "12: CLK_IN / 2 ^ (20 - 12)"]
    C2P12 = 12,
    #[doc = "13: CLK_IN / 2 ^ (20 - 13)"]
    C2P13 = 13,
    #[doc = "14: CLK_IN / 2 ^ (20 - 14)"]
    C2P14 = 14,
    #[doc = "15: CLK_IN / 2 ^ (20 - 15)"]
    C2P15 = 15,
}
impl From<FS_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_DIV_A) -> Self {
        variant as _
    }
}
impl FS_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_DIV_A {
        match self.bits {
            0 => FS_DIV_A::C2P0,
            1 => FS_DIV_A::C2P1,
            2 => FS_DIV_A::C2P2,
            3 => FS_DIV_A::C2P3,
            4 => FS_DIV_A::C2P4,
            5 => FS_DIV_A::C2P5,
            6 => FS_DIV_A::C2P6,
            7 => FS_DIV_A::C2P7,
            8 => FS_DIV_A::C2P8,
            9 => FS_DIV_A::C2P9,
            10 => FS_DIV_A::C2P10,
            11 => FS_DIV_A::C2P11,
            12 => FS_DIV_A::C2P12,
            13 => FS_DIV_A::C2P13,
            14 => FS_DIV_A::C2P14,
            15 => FS_DIV_A::C2P15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `C2P0`"]
    #[inline(always)]
    pub fn is_c2p0(&self) -> bool {
        *self == FS_DIV_A::C2P0
    }
    #[doc = "Checks if the value of the field is `C2P1`"]
    #[inline(always)]
    pub fn is_c2p1(&self) -> bool {
        *self == FS_DIV_A::C2P1
    }
    #[doc = "Checks if the value of the field is `C2P2`"]
    #[inline(always)]
    pub fn is_c2p2(&self) -> bool {
        *self == FS_DIV_A::C2P2
    }
    #[doc = "Checks if the value of the field is `C2P3`"]
    #[inline(always)]
    pub fn is_c2p3(&self) -> bool {
        *self == FS_DIV_A::C2P3
    }
    #[doc = "Checks if the value of the field is `C2P4`"]
    #[inline(always)]
    pub fn is_c2p4(&self) -> bool {
        *self == FS_DIV_A::C2P4
    }
    #[doc = "Checks if the value of the field is `C2P5`"]
    #[inline(always)]
    pub fn is_c2p5(&self) -> bool {
        *self == FS_DIV_A::C2P5
    }
    #[doc = "Checks if the value of the field is `C2P6`"]
    #[inline(always)]
    pub fn is_c2p6(&self) -> bool {
        *self == FS_DIV_A::C2P6
    }
    #[doc = "Checks if the value of the field is `C2P7`"]
    #[inline(always)]
    pub fn is_c2p7(&self) -> bool {
        *self == FS_DIV_A::C2P7
    }
    #[doc = "Checks if the value of the field is `C2P8`"]
    #[inline(always)]
    pub fn is_c2p8(&self) -> bool {
        *self == FS_DIV_A::C2P8
    }
    #[doc = "Checks if the value of the field is `C2P9`"]
    #[inline(always)]
    pub fn is_c2p9(&self) -> bool {
        *self == FS_DIV_A::C2P9
    }
    #[doc = "Checks if the value of the field is `C2P10`"]
    #[inline(always)]
    pub fn is_c2p10(&self) -> bool {
        *self == FS_DIV_A::C2P10
    }
    #[doc = "Checks if the value of the field is `C2P11`"]
    #[inline(always)]
    pub fn is_c2p11(&self) -> bool {
        *self == FS_DIV_A::C2P11
    }
    #[doc = "Checks if the value of the field is `C2P12`"]
    #[inline(always)]
    pub fn is_c2p12(&self) -> bool {
        *self == FS_DIV_A::C2P12
    }
    #[doc = "Checks if the value of the field is `C2P13`"]
    #[inline(always)]
    pub fn is_c2p13(&self) -> bool {
        *self == FS_DIV_A::C2P13
    }
    #[doc = "Checks if the value of the field is `C2P14`"]
    #[inline(always)]
    pub fn is_c2p14(&self) -> bool {
        *self == FS_DIV_A::C2P14
    }
    #[doc = "Checks if the value of the field is `C2P15`"]
    #[inline(always)]
    pub fn is_c2p15(&self) -> bool {
        *self == FS_DIV_A::C2P15
    }
}
#[doc = "Field `fs_div` writer - ADC Sample Frequency Divider"]
pub type FS_DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TP_CTRL0_SPEC, u8, FS_DIV_A, 4, O>;
impl<'a, const O: u8> FS_DIV_W<'a, O> {
    #[doc = "CLK_IN / 2 ^ (20 - 0)"]
    #[inline(always)]
    pub fn c2p0(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P0)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 1)"]
    #[inline(always)]
    pub fn c2p1(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P1)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 2)"]
    #[inline(always)]
    pub fn c2p2(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P2)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 3)"]
    #[inline(always)]
    pub fn c2p3(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P3)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 4)"]
    #[inline(always)]
    pub fn c2p4(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P4)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 5)"]
    #[inline(always)]
    pub fn c2p5(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P5)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 6)"]
    #[inline(always)]
    pub fn c2p6(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P6)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 7)"]
    #[inline(always)]
    pub fn c2p7(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P7)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 8)"]
    #[inline(always)]
    pub fn c2p8(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P8)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 9)"]
    #[inline(always)]
    pub fn c2p9(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P9)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 10)"]
    #[inline(always)]
    pub fn c2p10(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P10)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 11)"]
    #[inline(always)]
    pub fn c2p11(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P11)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 12)"]
    #[inline(always)]
    pub fn c2p12(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P12)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 13)"]
    #[inline(always)]
    pub fn c2p13(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P13)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 14)"]
    #[inline(always)]
    pub fn c2p14(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P14)
    }
    #[doc = "CLK_IN / 2 ^ (20 - 15)"]
    #[inline(always)]
    pub fn c2p15(self) -> &'a mut W {
        self.variant(FS_DIV_A::C2P15)
    }
}
#[doc = "Field `adc_clk_divider` reader - ADC Clock Divider (CLK_IN)"]
pub type ADC_CLK_DIVIDER_R = crate::FieldReader<u8, ADC_CLK_DIVIDER_A>;
#[doc = "ADC Clock Divider (CLK_IN)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_CLK_DIVIDER_A {
    #[doc = "0: CLK / 2"]
    C2 = 0,
    #[doc = "1: CLK / 3"]
    C3 = 1,
    #[doc = "2: CLK / 6"]
    C6 = 2,
    #[doc = "3: CLK / 1"]
    C1 = 3,
}
impl From<ADC_CLK_DIVIDER_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_CLK_DIVIDER_A) -> Self {
        variant as _
    }
}
impl ADC_CLK_DIVIDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_CLK_DIVIDER_A {
        match self.bits {
            0 => ADC_CLK_DIVIDER_A::C2,
            1 => ADC_CLK_DIVIDER_A::C3,
            2 => ADC_CLK_DIVIDER_A::C6,
            3 => ADC_CLK_DIVIDER_A::C1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `C2`"]
    #[inline(always)]
    pub fn is_c2(&self) -> bool {
        *self == ADC_CLK_DIVIDER_A::C2
    }
    #[doc = "Checks if the value of the field is `C3`"]
    #[inline(always)]
    pub fn is_c3(&self) -> bool {
        *self == ADC_CLK_DIVIDER_A::C3
    }
    #[doc = "Checks if the value of the field is `C6`"]
    #[inline(always)]
    pub fn is_c6(&self) -> bool {
        *self == ADC_CLK_DIVIDER_A::C6
    }
    #[doc = "Checks if the value of the field is `C1`"]
    #[inline(always)]
    pub fn is_c1(&self) -> bool {
        *self == ADC_CLK_DIVIDER_A::C1
    }
}
#[doc = "Field `adc_clk_divider` writer - ADC Clock Divider (CLK_IN)"]
pub type ADC_CLK_DIVIDER_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TP_CTRL0_SPEC, u8, ADC_CLK_DIVIDER_A, 2, O>;
impl<'a, const O: u8> ADC_CLK_DIVIDER_W<'a, O> {
    #[doc = "CLK / 2"]
    #[inline(always)]
    pub fn c2(self) -> &'a mut W {
        self.variant(ADC_CLK_DIVIDER_A::C2)
    }
    #[doc = "CLK / 3"]
    #[inline(always)]
    pub fn c3(self) -> &'a mut W {
        self.variant(ADC_CLK_DIVIDER_A::C3)
    }
    #[doc = "CLK / 6"]
    #[inline(always)]
    pub fn c6(self) -> &'a mut W {
        self.variant(ADC_CLK_DIVIDER_A::C6)
    }
    #[doc = "CLK / 1"]
    #[inline(always)]
    pub fn c1(self) -> &'a mut W {
        self.variant(ADC_CLK_DIVIDER_A::C1)
    }
}
#[doc = "Field `adc_first_dly_mode` reader - ADC First Convert Delay Mode Select"]
pub type ADC_FIRST_DLY_MODE_R = crate::BitReader<ADC_FIRST_DLY_MODE_A>;
#[doc = "ADC First Convert Delay Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_FIRST_DLY_MODE_A {
    #[doc = "0: CLK_IN / 16"]
    C16 = 0,
    #[doc = "1: CLK_IN / 16 * 256"]
    C16_256 = 1,
}
impl From<ADC_FIRST_DLY_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_FIRST_DLY_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_FIRST_DLY_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_FIRST_DLY_MODE_A {
        match self.bits {
            false => ADC_FIRST_DLY_MODE_A::C16,
            true => ADC_FIRST_DLY_MODE_A::C16_256,
        }
    }
    #[doc = "Checks if the value of the field is `C16`"]
    #[inline(always)]
    pub fn is_c16(&self) -> bool {
        *self == ADC_FIRST_DLY_MODE_A::C16
    }
    #[doc = "Checks if the value of the field is `C16_256`"]
    #[inline(always)]
    pub fn is_c16_256(&self) -> bool {
        *self == ADC_FIRST_DLY_MODE_A::C16_256
    }
}
#[doc = "Field `adc_first_dly_mode` writer - ADC First Convert Delay Mode Select"]
pub type ADC_FIRST_DLY_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_CTRL0_SPEC, ADC_FIRST_DLY_MODE_A, O>;
impl<'a, const O: u8> ADC_FIRST_DLY_MODE_W<'a, O> {
    #[doc = "CLK_IN / 16"]
    #[inline(always)]
    pub fn c16(self) -> &'a mut W {
        self.variant(ADC_FIRST_DLY_MODE_A::C16)
    }
    #[doc = "CLK_IN / 16 * 256"]
    #[inline(always)]
    pub fn c16_256(self) -> &'a mut W {
        self.variant(ADC_FIRST_DLY_MODE_A::C16_256)
    }
}
#[doc = "Field `adc_first_dly` reader - ADC First Convert Delay Time (T_FCDT) Setting"]
pub type ADC_FIRST_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_first_dly` writer - ADC First Convert Delay Time (T_FCDT) Setting"]
pub type ADC_FIRST_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TP_CTRL0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - Touch panel ADC acquire time"]
    #[inline(always)]
    pub fn tacq(&self) -> TACQ_R {
        TACQ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - ADC Sample Frequency Divider"]
    #[inline(always)]
    pub fn fs_div(&self) -> FS_DIV_R {
        FS_DIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - ADC Clock Divider (CLK_IN)"]
    #[inline(always)]
    pub fn adc_clk_divider(&self) -> ADC_CLK_DIVIDER_R {
        ADC_CLK_DIVIDER_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - ADC First Convert Delay Mode Select"]
    #[inline(always)]
    pub fn adc_first_dly_mode(&self) -> ADC_FIRST_DLY_MODE_R {
        ADC_FIRST_DLY_MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - ADC First Convert Delay Time (T_FCDT) Setting"]
    #[inline(always)]
    pub fn adc_first_dly(&self) -> ADC_FIRST_DLY_R {
        ADC_FIRST_DLY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Touch panel ADC acquire time"]
    #[inline(always)]
    #[must_use]
    pub fn tacq(&mut self) -> TACQ_W<0> {
        TACQ_W::new(self)
    }
    #[doc = "Bits 16:19 - ADC Sample Frequency Divider"]
    #[inline(always)]
    #[must_use]
    pub fn fs_div(&mut self) -> FS_DIV_W<16> {
        FS_DIV_W::new(self)
    }
    #[doc = "Bits 20:21 - ADC Clock Divider (CLK_IN)"]
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_divider(&mut self) -> ADC_CLK_DIVIDER_W<20> {
        ADC_CLK_DIVIDER_W::new(self)
    }
    #[doc = "Bit 23 - ADC First Convert Delay Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_first_dly_mode(&mut self) -> ADC_FIRST_DLY_MODE_W<23> {
        ADC_FIRST_DLY_MODE_W::new(self)
    }
    #[doc = "Bits 24:31 - ADC First Convert Delay Time (T_FCDT) Setting"]
    #[inline(always)]
    #[must_use]
    pub fn adc_first_dly(&mut self) -> ADC_FIRST_DLY_W<24> {
        ADC_FIRST_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_ctrl0](index.html) module"]
pub struct TP_CTRL0_SPEC;
impl crate::RegisterSpec for TP_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_ctrl0::R](R) reader structure"]
impl crate::Readable for TP_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_ctrl0::W](W) writer structure"]
impl crate::Writable for TP_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tp_ctrl0 to value 0"]
impl crate::Resettable for TP_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
