#[doc = "Register `RXDMA_CTRL` reader"]
pub struct R(crate::R<RXDMA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDMA_CTRL` writer"]
pub struct W(crate::W<RXDMA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_CTRL_SPEC>;
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
impl From<crate::W<RXDMA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timeout_threshold` reader - RXDMA Timeout Threshold\n\nUnit is 1 UART bit time"]
pub struct TIMEOUT_THRESHOLD_R(crate::FieldReader<u16>);
impl TIMEOUT_THRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMEOUT_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_THRESHOLD_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timeout_threshold` writer - RXDMA Timeout Threshold\n\nUnit is 1 UART bit time"]
pub struct TIMEOUT_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
#[doc = "Field `timeout_enable` reader - RXDMA Timeout Enable"]
pub struct TIMEOUT_ENABLE_R(crate::FieldReader<bool>);
impl TIMEOUT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timeout_enable` writer - RXDMA Timeout Enable"]
pub struct TIMEOUT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Set for AHB port burst supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AHB_BURST_MODE_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    INCR4 = 1,
    #[doc = "2: `10`"]
    INCR8 = 2,
    #[doc = "3: `11`"]
    INCR16 = 3,
}
impl From<AHB_BURST_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_BURST_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ahb_burst_mode` reader - Set for AHB port burst supported"]
pub struct AHB_BURST_MODE_R(crate::FieldReader<u8>);
impl AHB_BURST_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AHB_BURST_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_BURST_MODE_A {
        match self.bits {
            0 => AHB_BURST_MODE_A::SINGLE,
            1 => AHB_BURST_MODE_A::INCR4,
            2 => AHB_BURST_MODE_A::INCR8,
            3 => AHB_BURST_MODE_A::INCR16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == AHB_BURST_MODE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        **self == AHB_BURST_MODE_A::INCR4
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        **self == AHB_BURST_MODE_A::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        **self == AHB_BURST_MODE_A::INCR16
    }
}
impl core::ops::Deref for AHB_BURST_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ahb_burst_mode` writer - Set for AHB port burst supported"]
pub struct AHB_BURST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_BURST_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_BURST_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(AHB_BURST_MODE_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(AHB_BURST_MODE_A::INCR4)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(AHB_BURST_MODE_A::INCR8)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(AHB_BURST_MODE_A::INCR16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLK_SIZE_A {
    #[doc = "0: `0`"]
    B8 = 0,
    #[doc = "1: `1`"]
    B16 = 1,
    #[doc = "2: `10`"]
    B32 = 2,
    #[doc = "3: `11`"]
    B64 = 3,
}
impl From<BLK_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BLK_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `blk_size` reader - "]
pub struct BLK_SIZE_R(crate::FieldReader<u8>);
impl BLK_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLK_SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK_SIZE_A {
        match self.bits {
            0 => BLK_SIZE_A::B8,
            1 => BLK_SIZE_A::B16,
            2 => BLK_SIZE_A::B32,
            3 => BLK_SIZE_A::B64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B8`"]
    #[inline(always)]
    pub fn is_b8(&self) -> bool {
        **self == BLK_SIZE_A::B8
    }
    #[doc = "Checks if the value of the field is `B16`"]
    #[inline(always)]
    pub fn is_b16(&self) -> bool {
        **self == BLK_SIZE_A::B16
    }
    #[doc = "Checks if the value of the field is `B32`"]
    #[inline(always)]
    pub fn is_b32(&self) -> bool {
        **self == BLK_SIZE_A::B32
    }
    #[doc = "Checks if the value of the field is `B64`"]
    #[inline(always)]
    pub fn is_b64(&self) -> bool {
        **self == BLK_SIZE_A::B64
    }
}
impl core::ops::Deref for BLK_SIZE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `blk_size` writer - "]
pub struct BLK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLK_SIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn b8(self) -> &'a mut W {
        self.variant(BLK_SIZE_A::B8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn b16(self) -> &'a mut W {
        self.variant(BLK_SIZE_A::B16)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn b32(self) -> &'a mut W {
        self.variant(BLK_SIZE_A::B32)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn b64(self) -> &'a mut W {
        self.variant(BLK_SIZE_A::B64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    CONTINOUS = 0,
    #[doc = "1: `1`"]
    LIMITED = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `mode` reader - "]
pub struct MODE_R(crate::FieldReader<bool>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::CONTINOUS,
            true => MODE_A::LIMITED,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINOUS`"]
    #[inline(always)]
    pub fn is_continous(&self) -> bool {
        **self == MODE_A::CONTINOUS
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        **self == MODE_A::LIMITED
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mode` writer - "]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn continous(self) -> &'a mut W {
        self.variant(MODE_A::CONTINOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn limited(self) -> &'a mut W {
        self.variant(MODE_A::LIMITED)
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `enable` reader - "]
pub struct ENABLE_R(crate::FieldReader<bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLE,
            true => ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enable` writer - "]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:23 - RXDMA Timeout Threshold\n\nUnit is 1 UART bit time"]
    #[inline(always)]
    pub fn timeout_threshold(&self) -> TIMEOUT_THRESHOLD_R {
        TIMEOUT_THRESHOLD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 6 - RXDMA Timeout Enable"]
    #[inline(always)]
    pub fn timeout_enable(&self) -> TIMEOUT_ENABLE_R {
        TIMEOUT_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Set for AHB port burst supported"]
    #[inline(always)]
    pub fn ahb_burst_mode(&self) -> AHB_BURST_MODE_R {
        AHB_BURST_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn blk_size(&self) -> BLK_SIZE_R {
        BLK_SIZE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:23 - RXDMA Timeout Threshold\n\nUnit is 1 UART bit time"]
    #[inline(always)]
    pub fn timeout_threshold(&mut self) -> TIMEOUT_THRESHOLD_W {
        TIMEOUT_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 6 - RXDMA Timeout Enable"]
    #[inline(always)]
    pub fn timeout_enable(&mut self) -> TIMEOUT_ENABLE_W {
        TIMEOUT_ENABLE_W { w: self }
    }
    #[doc = "Bits 4:5 - Set for AHB port burst supported"]
    #[inline(always)]
    pub fn ahb_burst_mode(&mut self) -> AHB_BURST_MODE_W {
        AHB_BURST_MODE_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn blk_size(&mut self) -> BLK_SIZE_W {
        BLK_SIZE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART RXDMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_ctrl](index.html) module"]
pub struct RXDMA_CTRL_SPEC;
impl crate::RegisterSpec for RXDMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_ctrl::R](R) reader structure"]
impl crate::Readable for RXDMA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_ctrl::W](W) writer structure"]
impl crate::Writable for RXDMA_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDMA_CTRL to value 0"]
impl crate::Resettable for RXDMA_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
