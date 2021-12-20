#[doc = "Register `pd_pull1` reader"]
pub struct R(crate::R<PD_PULL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_PULL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_PULL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_PULL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pd_pull1` writer"]
pub struct W(crate::W<PD_PULL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_PULL1_SPEC>;
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
impl From<crate::W<PD_PULL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_PULL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PD22_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD22_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD22_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd22_pull` reader - "]
pub struct PD22_PULL_R(crate::FieldReader<u8, PD22_PULL_A>);
impl PD22_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD22_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD22_PULL_A {
        match self.bits {
            0 => PD22_PULL_A::PULL_DISABLE,
            1 => PD22_PULL_A::PULL_UP,
            2 => PD22_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD22_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD22_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD22_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD22_PULL_R {
    type Target = crate::FieldReader<u8, PD22_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd22_pull` writer - "]
pub struct PD22_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD22_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD22_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD22_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD22_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD22_PULL_A::PULL_DOWN)
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
pub enum PD21_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD21_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD21_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd21_pull` reader - "]
pub struct PD21_PULL_R(crate::FieldReader<u8, PD21_PULL_A>);
impl PD21_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD21_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD21_PULL_A {
        match self.bits {
            0 => PD21_PULL_A::PULL_DISABLE,
            1 => PD21_PULL_A::PULL_UP,
            2 => PD21_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD21_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD21_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD21_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD21_PULL_R {
    type Target = crate::FieldReader<u8, PD21_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd21_pull` writer - "]
pub struct PD21_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD21_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD21_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD21_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD21_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD21_PULL_A::PULL_DOWN)
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
pub enum PD20_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD20_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD20_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd20_pull` reader - "]
pub struct PD20_PULL_R(crate::FieldReader<u8, PD20_PULL_A>);
impl PD20_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD20_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD20_PULL_A {
        match self.bits {
            0 => PD20_PULL_A::PULL_DISABLE,
            1 => PD20_PULL_A::PULL_UP,
            2 => PD20_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD20_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD20_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD20_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD20_PULL_R {
    type Target = crate::FieldReader<u8, PD20_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd20_pull` writer - "]
pub struct PD20_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD20_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD20_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD20_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD20_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD20_PULL_A::PULL_DOWN)
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
pub enum PD19_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD19_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD19_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd19_pull` reader - "]
pub struct PD19_PULL_R(crate::FieldReader<u8, PD19_PULL_A>);
impl PD19_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD19_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD19_PULL_A {
        match self.bits {
            0 => PD19_PULL_A::PULL_DISABLE,
            1 => PD19_PULL_A::PULL_UP,
            2 => PD19_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD19_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD19_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD19_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD19_PULL_R {
    type Target = crate::FieldReader<u8, PD19_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd19_pull` writer - "]
pub struct PD19_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD19_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD19_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD19_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD19_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD19_PULL_A::PULL_DOWN)
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
pub enum PD18_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD18_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD18_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd18_pull` reader - "]
pub struct PD18_PULL_R(crate::FieldReader<u8, PD18_PULL_A>);
impl PD18_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD18_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD18_PULL_A {
        match self.bits {
            0 => PD18_PULL_A::PULL_DISABLE,
            1 => PD18_PULL_A::PULL_UP,
            2 => PD18_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD18_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD18_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD18_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD18_PULL_R {
    type Target = crate::FieldReader<u8, PD18_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd18_pull` writer - "]
pub struct PD18_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD18_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD18_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD18_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD18_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD18_PULL_A::PULL_DOWN)
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
pub enum PD17_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD17_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD17_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd17_pull` reader - "]
pub struct PD17_PULL_R(crate::FieldReader<u8, PD17_PULL_A>);
impl PD17_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD17_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD17_PULL_A {
        match self.bits {
            0 => PD17_PULL_A::PULL_DISABLE,
            1 => PD17_PULL_A::PULL_UP,
            2 => PD17_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD17_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD17_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD17_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD17_PULL_R {
    type Target = crate::FieldReader<u8, PD17_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd17_pull` writer - "]
pub struct PD17_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD17_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD17_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD17_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD17_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD17_PULL_A::PULL_DOWN)
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
pub enum PD16_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD16_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD16_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `pd16_pull` reader - "]
pub struct PD16_PULL_R(crate::FieldReader<u8, PD16_PULL_A>);
impl PD16_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD16_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD16_PULL_A {
        match self.bits {
            0 => PD16_PULL_A::PULL_DISABLE,
            1 => PD16_PULL_A::PULL_UP,
            2 => PD16_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PD16_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD16_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD16_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD16_PULL_R {
    type Target = crate::FieldReader<u8, PD16_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pd16_pull` writer - "]
pub struct PD16_PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD16_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD16_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD16_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD16_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD16_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pd22_pull(&self) -> PD22_PULL_R {
        PD22_PULL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pd21_pull(&self) -> PD21_PULL_R {
        PD21_PULL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pd20_pull(&self) -> PD20_PULL_R {
        PD20_PULL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pd19_pull(&self) -> PD19_PULL_R {
        PD19_PULL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pd18_pull(&self) -> PD18_PULL_R {
        PD18_PULL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pd17_pull(&self) -> PD17_PULL_R {
        PD17_PULL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pd16_pull(&self) -> PD16_PULL_R {
        PD16_PULL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pd22_pull(&mut self) -> PD22_PULL_W {
        PD22_PULL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pd21_pull(&mut self) -> PD21_PULL_W {
        PD21_PULL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pd20_pull(&mut self) -> PD20_PULL_W {
        PD20_PULL_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pd19_pull(&mut self) -> PD19_PULL_W {
        PD19_PULL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pd18_pull(&mut self) -> PD18_PULL_W {
        PD18_PULL_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pd17_pull(&mut self) -> PD17_PULL_W {
        PD17_PULL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pd16_pull(&mut self) -> PD16_PULL_W {
        PD16_PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Pull Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_pull1](index.html) module"]
pub struct PD_PULL1_SPEC;
impl crate::RegisterSpec for PD_PULL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_pull1::R](R) reader structure"]
impl crate::Readable for PD_PULL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_pull1::W](W) writer structure"]
impl crate::Writable for PD_PULL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pd_pull1 to value 0"]
impl crate::Resettable for PD_PULL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
