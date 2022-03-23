#[doc = "Register `PS_CTL` reader"]
pub struct R(crate::R<PS_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PS_CTL` writer"]
pub struct W(crate::W<PS_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PS_CTL_SPEC>;
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
impl From<crate::W<PS_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PS_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "clk250M counter ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK250M_CNT_RDY_A {
    #[doc = "0: `0`"]
    NOT_FINISH = 0,
    #[doc = "1: `1`"]
    FINISH = 1,
}
impl From<CLK250M_CNT_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: CLK250M_CNT_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK250M_CNT_RDY` reader - clk250M counter ready"]
pub struct CLK250M_CNT_RDY_R(crate::FieldReader<bool, CLK250M_CNT_RDY_A>);
impl CLK250M_CNT_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK250M_CNT_RDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK250M_CNT_RDY_A {
        match self.bits {
            false => CLK250M_CNT_RDY_A::NOT_FINISH,
            true => CLK250M_CNT_RDY_A::FINISH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FINISH`"]
    #[inline(always)]
    pub fn is_not_finish(&self) -> bool {
        **self == CLK250M_CNT_RDY_A::NOT_FINISH
    }
    #[doc = "Checks if the value of the field is `FINISH`"]
    #[inline(always)]
    pub fn is_finish(&self) -> bool {
        **self == CLK250M_CNT_RDY_A::FINISH
    }
}
impl core::ops::Deref for CLK250M_CNT_RDY_R {
    type Target = crate::FieldReader<bool, CLK250M_CNT_RDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK250M_CNT_RDY` writer - clk250M counter ready"]
pub struct CLK250M_CNT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK250M_CNT_RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK250M_CNT_RDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_finish(self) -> &'a mut W {
        self.variant(CLK250M_CNT_RDY_A::NOT_FINISH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn finish(self) -> &'a mut W {
        self.variant(CLK250M_CNT_RDY_A::FINISH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "psensor device sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PS_DEV_SEL_A {
    #[doc = "0: `0`"]
    PENSOR_0 = 0,
    #[doc = "1: `1`"]
    PENSOR_1 = 1,
}
impl From<PS_DEV_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_DEV_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PS_DEV_SEL` reader - psensor device sel"]
pub struct PS_DEV_SEL_R(crate::FieldReader<u8, PS_DEV_SEL_A>);
impl PS_DEV_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PS_DEV_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PS_DEV_SEL_A> {
        match self.bits {
            0 => Some(PS_DEV_SEL_A::PENSOR_0),
            1 => Some(PS_DEV_SEL_A::PENSOR_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PENSOR_0`"]
    #[inline(always)]
    pub fn is_pensor_0(&self) -> bool {
        **self == PS_DEV_SEL_A::PENSOR_0
    }
    #[doc = "Checks if the value of the field is `PENSOR_1`"]
    #[inline(always)]
    pub fn is_pensor_1(&self) -> bool {
        **self == PS_DEV_SEL_A::PENSOR_1
    }
}
impl core::ops::Deref for PS_DEV_SEL_R {
    type Target = crate::FieldReader<u8, PS_DEV_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS_DEV_SEL` writer - psensor device sel"]
pub struct PS_DEV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_DEV_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_DEV_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pensor_0(self) -> &'a mut W {
        self.variant(PS_DEV_SEL_A::PENSOR_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pensor_1(self) -> &'a mut W {
        self.variant(PS_DEV_SEL_A::PENSOR_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "OSC select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PS_EN_SEL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    RVT_CASECODE = 1,
    #[doc = "2: `10`"]
    LVT_CASECODE = 2,
    #[doc = "3: `11`"]
    RVT_NORMAL = 3,
}
impl From<PS_EN_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_EN_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PS_EN_SEL` reader - OSC select"]
pub struct PS_EN_SEL_R(crate::FieldReader<u8, PS_EN_SEL_A>);
impl PS_EN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PS_EN_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_EN_SEL_A {
        match self.bits {
            0 => PS_EN_SEL_A::DISABLE,
            1 => PS_EN_SEL_A::RVT_CASECODE,
            2 => PS_EN_SEL_A::LVT_CASECODE,
            3 => PS_EN_SEL_A::RVT_NORMAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PS_EN_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RVT_CASECODE`"]
    #[inline(always)]
    pub fn is_rvt_casecode(&self) -> bool {
        **self == PS_EN_SEL_A::RVT_CASECODE
    }
    #[doc = "Checks if the value of the field is `LVT_CASECODE`"]
    #[inline(always)]
    pub fn is_lvt_casecode(&self) -> bool {
        **self == PS_EN_SEL_A::LVT_CASECODE
    }
    #[doc = "Checks if the value of the field is `RVT_NORMAL`"]
    #[inline(always)]
    pub fn is_rvt_normal(&self) -> bool {
        **self == PS_EN_SEL_A::RVT_NORMAL
    }
}
impl core::ops::Deref for PS_EN_SEL_R {
    type Target = crate::FieldReader<u8, PS_EN_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS_EN_SEL` writer - OSC select"]
pub struct PS_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_EN_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_EN_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PS_EN_SEL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rvt_casecode(self) -> &'a mut W {
        self.variant(PS_EN_SEL_A::RVT_CASECODE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lvt_casecode(self) -> &'a mut W {
        self.variant(PS_EN_SEL_A::LVT_CASECODE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rvt_normal(self) -> &'a mut W {
        self.variant(PS_EN_SEL_A::RVT_NORMAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "clk num period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PS_N_PRD_A {
    #[doc = "0: `0`"]
    P4 = 0,
    #[doc = "1: `1`"]
    P8 = 1,
    #[doc = "2: `10`"]
    P16 = 2,
    #[doc = "3: `11`"]
    P32 = 3,
    #[doc = "4: `100`"]
    P64 = 4,
    #[doc = "5: `101`"]
    P128 = 5,
    #[doc = "6: `110`"]
    P256 = 6,
    #[doc = "7: `111`"]
    P512 = 7,
}
impl From<PS_N_PRD_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_N_PRD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PS_N_PRD` reader - clk num period"]
pub struct PS_N_PRD_R(crate::FieldReader<u8, PS_N_PRD_A>);
impl PS_N_PRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PS_N_PRD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_N_PRD_A {
        match self.bits {
            0 => PS_N_PRD_A::P4,
            1 => PS_N_PRD_A::P8,
            2 => PS_N_PRD_A::P16,
            3 => PS_N_PRD_A::P32,
            4 => PS_N_PRD_A::P64,
            5 => PS_N_PRD_A::P128,
            6 => PS_N_PRD_A::P256,
            7 => PS_N_PRD_A::P512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P4`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        **self == PS_N_PRD_A::P4
    }
    #[doc = "Checks if the value of the field is `P8`"]
    #[inline(always)]
    pub fn is_p8(&self) -> bool {
        **self == PS_N_PRD_A::P8
    }
    #[doc = "Checks if the value of the field is `P16`"]
    #[inline(always)]
    pub fn is_p16(&self) -> bool {
        **self == PS_N_PRD_A::P16
    }
    #[doc = "Checks if the value of the field is `P32`"]
    #[inline(always)]
    pub fn is_p32(&self) -> bool {
        **self == PS_N_PRD_A::P32
    }
    #[doc = "Checks if the value of the field is `P64`"]
    #[inline(always)]
    pub fn is_p64(&self) -> bool {
        **self == PS_N_PRD_A::P64
    }
    #[doc = "Checks if the value of the field is `P128`"]
    #[inline(always)]
    pub fn is_p128(&self) -> bool {
        **self == PS_N_PRD_A::P128
    }
    #[doc = "Checks if the value of the field is `P256`"]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        **self == PS_N_PRD_A::P256
    }
    #[doc = "Checks if the value of the field is `P512`"]
    #[inline(always)]
    pub fn is_p512(&self) -> bool {
        **self == PS_N_PRD_A::P512
    }
}
impl core::ops::Deref for PS_N_PRD_R {
    type Target = crate::FieldReader<u8, PS_N_PRD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS_N_PRD` writer - clk num period"]
pub struct PS_N_PRD_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_N_PRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_N_PRD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut W {
        self.variant(PS_N_PRD_A::P4)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn p8(self) -> &'a mut W {
        self.variant(PS_N_PRD_A::P8)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn p16(self) -> &'a mut W {
        self.variant(PS_N_PRD_A::P16)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn p32(self) -> &'a mut W {
        self.variant(PS_N_PRD_A::P32)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn p64(self) -> &'a mut W {
        self.variant(PS_N_PRD_A::P64)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn p128(self) -> &'a mut W {
        self.variant(PS_N_PRD_A::P128)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn p256(self) -> &'a mut W {
        self.variant(PS_N_PRD_A::P256)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn p512(self) -> &'a mut W {
        self.variant(PS_N_PRD_A::P512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "psensor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS_EN` reader - psensor enable"]
pub struct PS_EN_R(crate::FieldReader<bool, PS_EN_A>);
impl PS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PS_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_EN_A {
        match self.bits {
            false => PS_EN_A::DISABLE,
            true => PS_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PS_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PS_EN_A::ENABLE
    }
}
impl core::ops::Deref for PS_EN_R {
    type Target = crate::FieldReader<bool, PS_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS_EN` writer - psensor enable"]
pub struct PS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PS_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PS_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - clk250M counter ready"]
    #[inline(always)]
    pub fn clk250m_cnt_rdy(&self) -> CLK250M_CNT_RDY_R {
        CLK250M_CNT_RDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - psensor device sel"]
    #[inline(always)]
    pub fn ps_dev_sel(&self) -> PS_DEV_SEL_R {
        PS_DEV_SEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - OSC select"]
    #[inline(always)]
    pub fn ps_en_sel(&self) -> PS_EN_SEL_R {
        PS_EN_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 1:3 - clk num period"]
    #[inline(always)]
    pub fn ps_n_prd(&self) -> PS_N_PRD_R {
        PS_N_PRD_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - psensor enable"]
    #[inline(always)]
    pub fn ps_en(&self) -> PS_EN_R {
        PS_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - clk250M counter ready"]
    #[inline(always)]
    pub fn clk250m_cnt_rdy(&mut self) -> CLK250M_CNT_RDY_W {
        CLK250M_CNT_RDY_W { w: self }
    }
    #[doc = "Bits 8:10 - psensor device sel"]
    #[inline(always)]
    pub fn ps_dev_sel(&mut self) -> PS_DEV_SEL_W {
        PS_DEV_SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - OSC select"]
    #[inline(always)]
    pub fn ps_en_sel(&mut self) -> PS_EN_SEL_W {
        PS_EN_SEL_W { w: self }
    }
    #[doc = "Bits 1:3 - clk num period"]
    #[inline(always)]
    pub fn ps_n_prd(&mut self) -> PS_N_PRD_W {
        PS_N_PRD_W { w: self }
    }
    #[doc = "Bit 0 - psensor enable"]
    #[inline(always)]
    pub fn ps_en(&mut self) -> PS_EN_W {
        PS_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Psensor control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_ctl](index.html) module"]
pub struct PS_CTL_SPEC;
impl crate::RegisterSpec for PS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps_ctl::R](R) reader structure"]
impl crate::Readable for PS_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ps_ctl::W](W) writer structure"]
impl crate::Writable for PS_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PS_CTL to value 0"]
impl crate::Resettable for PS_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
