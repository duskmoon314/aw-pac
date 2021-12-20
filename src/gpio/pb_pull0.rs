#[doc = "Register `pb_pull0` reader"]
pub struct R(crate::R<PB_PULL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_PULL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_PULL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_PULL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pb_pull0` writer"]
pub struct W(crate::W<PB_PULL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_PULL0_SPEC>;
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
impl From<crate::W<PB_PULL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_PULL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PB12_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB12_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB12_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb12_pull` reader - "]
pub struct PB12_PULL_R(crate::FieldReader<u8, PB12_PULL_A>);
impl PB12_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB12_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB12_PULL_A {
        match self.bits {
            0 => PB12_PULL_A::PULL_DISABLE,
            1 => PB12_PULL_A::PULL_UP,
            2 => PB12_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB12_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB12_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB12_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB12_PULL_R {
    type Target = crate::FieldReader<u8, PB12_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb12_pull` writer - "]
pub struct PB12_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB12_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB12_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB12_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB12_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB12_PULL_A::PULL_DOWN)
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
pub enum PB11_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB11_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB11_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb11_pull` reader - "]
pub struct PB11_PULL_R(crate::FieldReader<u8, PB11_PULL_A>);
impl PB11_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB11_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB11_PULL_A {
        match self.bits {
            0 => PB11_PULL_A::PULL_DISABLE,
            1 => PB11_PULL_A::PULL_UP,
            2 => PB11_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB11_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB11_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB11_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB11_PULL_R {
    type Target = crate::FieldReader<u8, PB11_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb11_pull` writer - "]
pub struct PB11_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB11_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB11_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB11_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB11_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB11_PULL_A::PULL_DOWN)
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
pub enum PB10_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB10_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB10_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb10_pull` reader - "]
pub struct PB10_PULL_R(crate::FieldReader<u8, PB10_PULL_A>);
impl PB10_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB10_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB10_PULL_A {
        match self.bits {
            0 => PB10_PULL_A::PULL_DISABLE,
            1 => PB10_PULL_A::PULL_UP,
            2 => PB10_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB10_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB10_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB10_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB10_PULL_R {
    type Target = crate::FieldReader<u8, PB10_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb10_pull` writer - "]
pub struct PB10_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB10_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB10_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB10_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB10_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB10_PULL_A::PULL_DOWN)
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
pub enum PB9_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB9_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB9_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb9_pull` reader - "]
pub struct PB9_PULL_R(crate::FieldReader<u8, PB9_PULL_A>);
impl PB9_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB9_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB9_PULL_A {
        match self.bits {
            0 => PB9_PULL_A::PULL_DISABLE,
            1 => PB9_PULL_A::PULL_UP,
            2 => PB9_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB9_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB9_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB9_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB9_PULL_R {
    type Target = crate::FieldReader<u8, PB9_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb9_pull` writer - "]
pub struct PB9_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB9_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB9_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB9_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB9_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB9_PULL_A::PULL_DOWN)
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
pub enum PB8_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB8_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB8_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb8_pull` reader - "]
pub struct PB8_PULL_R(crate::FieldReader<u8, PB8_PULL_A>);
impl PB8_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB8_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB8_PULL_A {
        match self.bits {
            0 => PB8_PULL_A::PULL_DISABLE,
            1 => PB8_PULL_A::PULL_UP,
            2 => PB8_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB8_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB8_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB8_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB8_PULL_R {
    type Target = crate::FieldReader<u8, PB8_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb8_pull` writer - "]
pub struct PB8_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB8_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB8_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB8_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB8_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB8_PULL_A::PULL_DOWN)
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
pub enum PB7_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB7_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB7_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb7_pull` reader - "]
pub struct PB7_PULL_R(crate::FieldReader<u8, PB7_PULL_A>);
impl PB7_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB7_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB7_PULL_A {
        match self.bits {
            0 => PB7_PULL_A::PULL_DISABLE,
            1 => PB7_PULL_A::PULL_UP,
            2 => PB7_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB7_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB7_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB7_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB7_PULL_R {
    type Target = crate::FieldReader<u8, PB7_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb7_pull` writer - "]
pub struct PB7_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB7_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB7_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB7_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB7_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB7_PULL_A::PULL_DOWN)
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
pub enum PB6_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB6_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB6_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb6_pull` reader - "]
pub struct PB6_PULL_R(crate::FieldReader<u8, PB6_PULL_A>);
impl PB6_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB6_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB6_PULL_A {
        match self.bits {
            0 => PB6_PULL_A::PULL_DISABLE,
            1 => PB6_PULL_A::PULL_UP,
            2 => PB6_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB6_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB6_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB6_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB6_PULL_R {
    type Target = crate::FieldReader<u8, PB6_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb6_pull` writer - "]
pub struct PB6_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB6_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB6_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB6_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB6_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB6_PULL_A::PULL_DOWN)
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
pub enum PB5_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB5_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB5_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb5_pull` reader - "]
pub struct PB5_PULL_R(crate::FieldReader<u8, PB5_PULL_A>);
impl PB5_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB5_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB5_PULL_A {
        match self.bits {
            0 => PB5_PULL_A::PULL_DISABLE,
            1 => PB5_PULL_A::PULL_UP,
            2 => PB5_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB5_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB5_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB5_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB5_PULL_R {
    type Target = crate::FieldReader<u8, PB5_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb5_pull` writer - "]
pub struct PB5_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB5_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB5_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB5_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB5_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB5_PULL_A::PULL_DOWN)
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
pub enum PB4_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB4_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB4_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb4_pull` reader - "]
pub struct PB4_PULL_R(crate::FieldReader<u8, PB4_PULL_A>);
impl PB4_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB4_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB4_PULL_A {
        match self.bits {
            0 => PB4_PULL_A::PULL_DISABLE,
            1 => PB4_PULL_A::PULL_UP,
            2 => PB4_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB4_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB4_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB4_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB4_PULL_R {
    type Target = crate::FieldReader<u8, PB4_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb4_pull` writer - "]
pub struct PB4_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB4_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB4_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB4_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB4_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB4_PULL_A::PULL_DOWN)
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
pub enum PB3_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB3_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB3_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb3_pull` reader - "]
pub struct PB3_PULL_R(crate::FieldReader<u8, PB3_PULL_A>);
impl PB3_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB3_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB3_PULL_A {
        match self.bits {
            0 => PB3_PULL_A::PULL_DISABLE,
            1 => PB3_PULL_A::PULL_UP,
            2 => PB3_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB3_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB3_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB3_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB3_PULL_R {
    type Target = crate::FieldReader<u8, PB3_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb3_pull` writer - "]
pub struct PB3_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB3_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB3_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB3_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB3_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB3_PULL_A::PULL_DOWN)
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
pub enum PB2_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB2_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB2_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb2_pull` reader - "]
pub struct PB2_PULL_R(crate::FieldReader<u8, PB2_PULL_A>);
impl PB2_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB2_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB2_PULL_A {
        match self.bits {
            0 => PB2_PULL_A::PULL_DISABLE,
            1 => PB2_PULL_A::PULL_UP,
            2 => PB2_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB2_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB2_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB2_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB2_PULL_R {
    type Target = crate::FieldReader<u8, PB2_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb2_pull` writer - "]
pub struct PB2_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB2_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB2_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB2_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB2_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB2_PULL_A::PULL_DOWN)
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
pub enum PB1_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB1_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB1_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb1_pull` reader - "]
pub struct PB1_PULL_R(crate::FieldReader<u8, PB1_PULL_A>);
impl PB1_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB1_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB1_PULL_A {
        match self.bits {
            0 => PB1_PULL_A::PULL_DISABLE,
            1 => PB1_PULL_A::PULL_UP,
            2 => PB1_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB1_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB1_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB1_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB1_PULL_R {
    type Target = crate::FieldReader<u8, PB1_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb1_pull` writer - "]
pub struct PB1_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB1_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB1_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB1_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB1_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB1_PULL_A::PULL_DOWN)
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
pub enum PB0_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PB0_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PB0_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pb0_pull` reader - "]
pub struct PB0_PULL_R(crate::FieldReader<u8, PB0_PULL_A>);
impl PB0_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB0_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PB0_PULL_A {
        match self.bits {
            0 => PB0_PULL_A::PULL_DISABLE,
            1 => PB0_PULL_A::PULL_UP,
            2 => PB0_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PB0_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PB0_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PB0_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PB0_PULL_R {
    type Target = crate::FieldReader<u8, PB0_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pb0_pull` writer - "]
pub struct PB0_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB0_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PB0_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PB0_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PB0_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PB0_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pb12_pull(&self) -> PB12_PULL_R {
        PB12_PULL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pb11_pull(&self) -> PB11_PULL_R {
        PB11_PULL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pb10_pull(&self) -> PB10_PULL_R {
        PB10_PULL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pb9_pull(&self) -> PB9_PULL_R {
        PB9_PULL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pb8_pull(&self) -> PB8_PULL_R {
        PB8_PULL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pb7_pull(&self) -> PB7_PULL_R {
        PB7_PULL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pb6_pull(&self) -> PB6_PULL_R {
        PB6_PULL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pb5_pull(&self) -> PB5_PULL_R {
        PB5_PULL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pb4_pull(&self) -> PB4_PULL_R {
        PB4_PULL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pb3_pull(&self) -> PB3_PULL_R {
        PB3_PULL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pb2_pull(&self) -> PB2_PULL_R {
        PB2_PULL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pb1_pull(&self) -> PB1_PULL_R {
        PB1_PULL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pb0_pull(&self) -> PB0_PULL_R {
        PB0_PULL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pb12_pull(&mut self) -> PB12_PULL_W {
        PB12_PULL_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pb11_pull(&mut self) -> PB11_PULL_W {
        PB11_PULL_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pb10_pull(&mut self) -> PB10_PULL_W {
        PB10_PULL_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pb9_pull(&mut self) -> PB9_PULL_W {
        PB9_PULL_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pb8_pull(&mut self) -> PB8_PULL_W {
        PB8_PULL_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pb7_pull(&mut self) -> PB7_PULL_W {
        PB7_PULL_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pb6_pull(&mut self) -> PB6_PULL_W {
        PB6_PULL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pb5_pull(&mut self) -> PB5_PULL_W {
        PB5_PULL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pb4_pull(&mut self) -> PB4_PULL_W {
        PB4_PULL_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pb3_pull(&mut self) -> PB3_PULL_W {
        PB3_PULL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pb2_pull(&mut self) -> PB2_PULL_W {
        PB2_PULL_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pb1_pull(&mut self) -> PB1_PULL_W {
        PB1_PULL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pb0_pull(&mut self) -> PB0_PULL_W {
        PB0_PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PB Pull Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_pull0](index.html) module"]
pub struct PB_PULL0_SPEC;
impl crate::RegisterSpec for PB_PULL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_pull0::R](R) reader structure"]
impl crate::Readable for PB_PULL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_pull0::W](W) writer structure"]
impl crate::Writable for PB_PULL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pb_pull0 to value 0"]
impl crate::Resettable for PB_PULL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
