#[doc = "Register `pd_pull0` reader"]
pub struct R(crate::R<PD_PULL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_PULL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_PULL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_PULL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pd_pull0` writer"]
pub struct W(crate::W<PD_PULL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_PULL0_SPEC>;
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
impl From<crate::W<PD_PULL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_PULL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD15_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD15_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD15_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd15_pull` reader - "]
pub struct PD15_PULL_R(crate::FieldReader<u8, PD15_PULL_A>);
impl PD15_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD15_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD15_PULL_A {
        match self.bits {
            0 => PD15_PULL_A::PULL_DISABLE,
            1 => PD15_PULL_A::PULL_UP,
            2 => PD15_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD15_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD15_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD15_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD15_PULL_R {
    type Target = crate::FieldReader<u8, PD15_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd15_pull` writer - "]
pub struct PD15_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD15_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD15_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD15_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD15_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD15_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD14_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD14_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD14_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd14_pull` reader - "]
pub struct PD14_PULL_R(crate::FieldReader<u8, PD14_PULL_A>);
impl PD14_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD14_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD14_PULL_A {
        match self.bits {
            0 => PD14_PULL_A::PULL_DISABLE,
            1 => PD14_PULL_A::PULL_UP,
            2 => PD14_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD14_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD14_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD14_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD14_PULL_R {
    type Target = crate::FieldReader<u8, PD14_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd14_pull` writer - "]
pub struct PD14_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD14_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD14_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD14_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD14_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD14_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD13_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD13_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD13_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd13_pull` reader - "]
pub struct PD13_PULL_R(crate::FieldReader<u8, PD13_PULL_A>);
impl PD13_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD13_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD13_PULL_A {
        match self.bits {
            0 => PD13_PULL_A::PULL_DISABLE,
            1 => PD13_PULL_A::PULL_UP,
            2 => PD13_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD13_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD13_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD13_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD13_PULL_R {
    type Target = crate::FieldReader<u8, PD13_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd13_pull` writer - "]
pub struct PD13_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD13_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD13_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD13_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD13_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD13_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD12_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD12_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD12_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd12_pull` reader - "]
pub struct PD12_PULL_R(crate::FieldReader<u8, PD12_PULL_A>);
impl PD12_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD12_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD12_PULL_A {
        match self.bits {
            0 => PD12_PULL_A::PULL_DISABLE,
            1 => PD12_PULL_A::PULL_UP,
            2 => PD12_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD12_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD12_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD12_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD12_PULL_R {
    type Target = crate::FieldReader<u8, PD12_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd12_pull` writer - "]
pub struct PD12_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD12_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD12_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD12_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD12_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD12_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD11_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD11_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD11_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd11_pull` reader - "]
pub struct PD11_PULL_R(crate::FieldReader<u8, PD11_PULL_A>);
impl PD11_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD11_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD11_PULL_A {
        match self.bits {
            0 => PD11_PULL_A::PULL_DISABLE,
            1 => PD11_PULL_A::PULL_UP,
            2 => PD11_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD11_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD11_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD11_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD11_PULL_R {
    type Target = crate::FieldReader<u8, PD11_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd11_pull` writer - "]
pub struct PD11_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD11_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD11_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD11_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD11_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD11_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD10_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD10_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD10_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd10_pull` reader - "]
pub struct PD10_PULL_R(crate::FieldReader<u8, PD10_PULL_A>);
impl PD10_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD10_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD10_PULL_A {
        match self.bits {
            0 => PD10_PULL_A::PULL_DISABLE,
            1 => PD10_PULL_A::PULL_UP,
            2 => PD10_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD10_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD10_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD10_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD10_PULL_R {
    type Target = crate::FieldReader<u8, PD10_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd10_pull` writer - "]
pub struct PD10_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD10_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD10_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD10_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD10_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD10_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD9_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD9_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD9_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd9_pull` reader - "]
pub struct PD9_PULL_R(crate::FieldReader<u8, PD9_PULL_A>);
impl PD9_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD9_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD9_PULL_A {
        match self.bits {
            0 => PD9_PULL_A::PULL_DISABLE,
            1 => PD9_PULL_A::PULL_UP,
            2 => PD9_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD9_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD9_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD9_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD9_PULL_R {
    type Target = crate::FieldReader<u8, PD9_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd9_pull` writer - "]
pub struct PD9_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD9_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD9_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD9_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD9_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD9_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD8_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD8_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD8_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd8_pull` reader - "]
pub struct PD8_PULL_R(crate::FieldReader<u8, PD8_PULL_A>);
impl PD8_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD8_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD8_PULL_A {
        match self.bits {
            0 => PD8_PULL_A::PULL_DISABLE,
            1 => PD8_PULL_A::PULL_UP,
            2 => PD8_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD8_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD8_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD8_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD8_PULL_R {
    type Target = crate::FieldReader<u8, PD8_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd8_pull` writer - "]
pub struct PD8_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD8_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD8_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD8_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD8_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD8_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD7_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD7_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD7_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd7_pull` reader - "]
pub struct PD7_PULL_R(crate::FieldReader<u8, PD7_PULL_A>);
impl PD7_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD7_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD7_PULL_A {
        match self.bits {
            0 => PD7_PULL_A::PULL_DISABLE,
            1 => PD7_PULL_A::PULL_UP,
            2 => PD7_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD7_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD7_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD7_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD7_PULL_R {
    type Target = crate::FieldReader<u8, PD7_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd7_pull` writer - "]
pub struct PD7_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD7_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD7_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD7_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD7_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD7_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD6_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD6_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD6_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd6_pull` reader - "]
pub struct PD6_PULL_R(crate::FieldReader<u8, PD6_PULL_A>);
impl PD6_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD6_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD6_PULL_A {
        match self.bits {
            0 => PD6_PULL_A::PULL_DISABLE,
            1 => PD6_PULL_A::PULL_UP,
            2 => PD6_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD6_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD6_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD6_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD6_PULL_R {
    type Target = crate::FieldReader<u8, PD6_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd6_pull` writer - "]
pub struct PD6_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD6_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD6_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD6_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD6_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD6_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD5_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD5_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD5_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd5_pull` reader - "]
pub struct PD5_PULL_R(crate::FieldReader<u8, PD5_PULL_A>);
impl PD5_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD5_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD5_PULL_A {
        match self.bits {
            0 => PD5_PULL_A::PULL_DISABLE,
            1 => PD5_PULL_A::PULL_UP,
            2 => PD5_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD5_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD5_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD5_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD5_PULL_R {
    type Target = crate::FieldReader<u8, PD5_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd5_pull` writer - "]
pub struct PD5_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD5_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD5_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD5_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD5_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD5_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD4_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD4_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD4_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd4_pull` reader - "]
pub struct PD4_PULL_R(crate::FieldReader<u8, PD4_PULL_A>);
impl PD4_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD4_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD4_PULL_A {
        match self.bits {
            0 => PD4_PULL_A::PULL_DISABLE,
            1 => PD4_PULL_A::PULL_UP,
            2 => PD4_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD4_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD4_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD4_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD4_PULL_R {
    type Target = crate::FieldReader<u8, PD4_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd4_pull` writer - "]
pub struct PD4_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD4_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD4_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD4_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD4_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD4_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD3_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD3_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD3_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd3_pull` reader - "]
pub struct PD3_PULL_R(crate::FieldReader<u8, PD3_PULL_A>);
impl PD3_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD3_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD3_PULL_A {
        match self.bits {
            0 => PD3_PULL_A::PULL_DISABLE,
            1 => PD3_PULL_A::PULL_UP,
            2 => PD3_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD3_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD3_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD3_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD3_PULL_R {
    type Target = crate::FieldReader<u8, PD3_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd3_pull` writer - "]
pub struct PD3_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD3_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD3_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD3_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD3_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD3_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD2_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD2_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD2_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd2_pull` reader - "]
pub struct PD2_PULL_R(crate::FieldReader<u8, PD2_PULL_A>);
impl PD2_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD2_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD2_PULL_A {
        match self.bits {
            0 => PD2_PULL_A::PULL_DISABLE,
            1 => PD2_PULL_A::PULL_UP,
            2 => PD2_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD2_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD2_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD2_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD2_PULL_R {
    type Target = crate::FieldReader<u8, PD2_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd2_pull` writer - "]
pub struct PD2_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD2_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD2_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD2_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD2_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD2_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD1_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD1_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD1_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd1_pull` reader - "]
pub struct PD1_PULL_R(crate::FieldReader<u8, PD1_PULL_A>);
impl PD1_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD1_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD1_PULL_A {
        match self.bits {
            0 => PD1_PULL_A::PULL_DISABLE,
            1 => PD1_PULL_A::PULL_UP,
            2 => PD1_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD1_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD1_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD1_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD1_PULL_R {
    type Target = crate::FieldReader<u8, PD1_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd1_pull` writer - "]
pub struct PD1_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD1_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD1_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD1_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD1_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD1_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD0_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD0_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD0_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd0_pull` reader - "]
pub struct PD0_PULL_R(crate::FieldReader<u8, PD0_PULL_A>);
impl PD0_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD0_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD0_PULL_A {
        match self.bits {
            0 => PD0_PULL_A::PULL_DISABLE,
            1 => PD0_PULL_A::PULL_UP,
            2 => PD0_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD0_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD0_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD0_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD0_PULL_R {
    type Target = crate::FieldReader<u8, PD0_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd0_pull` writer - "]
pub struct PD0_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD0_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD0_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD0_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD0_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD0_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pd15_pull(&self) -> PD15_PULL_R {
        PD15_PULL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pd14_pull(&self) -> PD14_PULL_R {
        PD14_PULL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pd13_pull(&self) -> PD13_PULL_R {
        PD13_PULL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pd12_pull(&self) -> PD12_PULL_R {
        PD12_PULL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pd11_pull(&self) -> PD11_PULL_R {
        PD11_PULL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pd10_pull(&self) -> PD10_PULL_R {
        PD10_PULL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pd9_pull(&self) -> PD9_PULL_R {
        PD9_PULL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pd8_pull(&self) -> PD8_PULL_R {
        PD8_PULL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pd7_pull(&self) -> PD7_PULL_R {
        PD7_PULL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pd6_pull(&self) -> PD6_PULL_R {
        PD6_PULL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pd5_pull(&self) -> PD5_PULL_R {
        PD5_PULL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pd4_pull(&self) -> PD4_PULL_R {
        PD4_PULL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pd3_pull(&self) -> PD3_PULL_R {
        PD3_PULL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pd2_pull(&self) -> PD2_PULL_R {
        PD2_PULL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pd1_pull(&self) -> PD1_PULL_R {
        PD1_PULL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pd0_pull(&self) -> PD0_PULL_R {
        PD0_PULL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pd15_pull(&mut self) -> PD15_PULL_W {
        PD15_PULL_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pd14_pull(&mut self) -> PD14_PULL_W {
        PD14_PULL_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pd13_pull(&mut self) -> PD13_PULL_W {
        PD13_PULL_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pd12_pull(&mut self) -> PD12_PULL_W {
        PD12_PULL_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pd11_pull(&mut self) -> PD11_PULL_W {
        PD11_PULL_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pd10_pull(&mut self) -> PD10_PULL_W {
        PD10_PULL_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pd9_pull(&mut self) -> PD9_PULL_W {
        PD9_PULL_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pd8_pull(&mut self) -> PD8_PULL_W {
        PD8_PULL_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pd7_pull(&mut self) -> PD7_PULL_W {
        PD7_PULL_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pd6_pull(&mut self) -> PD6_PULL_W {
        PD6_PULL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pd5_pull(&mut self) -> PD5_PULL_W {
        PD5_PULL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pd4_pull(&mut self) -> PD4_PULL_W {
        PD4_PULL_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pd3_pull(&mut self) -> PD3_PULL_W {
        PD3_PULL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pd2_pull(&mut self) -> PD2_PULL_W {
        PD2_PULL_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pd1_pull(&mut self) -> PD1_PULL_W {
        PD1_PULL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pd0_pull(&mut self) -> PD0_PULL_W {
        PD0_PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Pull Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_pull0](index.html) module"]
pub struct PD_PULL0_SPEC;
impl crate::RegisterSpec for PD_PULL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_pull0::R](R) reader structure"]
impl crate::Readable for PD_PULL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_pull0::W](W) writer structure"]
impl crate::Writable for PD_PULL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pd_pull0 to value 0"]
impl crate::Resettable for PD_PULL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
