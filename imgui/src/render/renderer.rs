use std::collections::HashMap;

/// An opaque texture identifier
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct TextureId(u64);

impl TextureId {
    /// Creates a new texture id with the given identifier.
    #[inline]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the id of the TextureId.
    #[inline]
    pub const fn id(self) -> u64 {
        self.0
    }
}

impl From<u64> for TextureId {
    #[inline]
    fn from(id: u64) -> Self {
        TextureId(id)
    }
}

impl<T> From<*const T> for TextureId {
    #[inline]
    fn from(ptr: *const T) -> Self {
        TextureId(ptr as u64)
    }
}

impl<T> From<*mut T> for TextureId {
    #[inline]
    fn from(ptr: *mut T) -> Self {
        TextureId(ptr as u64)
    }
}

#[test]
fn test_texture_id_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<TextureId>(),
        mem::size_of::<sys::ImTextureID>()
    );
    assert_eq!(
        mem::align_of::<TextureId>(),
        mem::align_of::<sys::ImTextureID>()
    );
}

/// Generic texture mapping for use by renderers.
#[derive(Debug)]
pub struct Textures<T> {
    textures: HashMap<u64, T>,
    next: u64,
}

/// We manually impl Default as `#[derive(Default)]`
/// incorrectly requires `T: Default` which is
/// not necessary at all.
impl<T> Default for Textures<T> {
    fn default() -> Self {
        Self {
            textures: Default::default(),
            next: Default::default(),
        }
    }
}

impl<T> Textures<T> {
    // TODO: hasher like rustc_hash::FxHashMap or something would let this be
    // `const fn`
    pub fn new() -> Self {
        Textures {
            textures: HashMap::new(),
            next: 0,
        }
    }

    pub fn insert(&mut self, texture: T) -> TextureId {
        let id = self.next;
        self.textures.insert(id, texture);
        self.next += 1;
        TextureId::from(id)
    }

    pub fn replace(&mut self, id: TextureId, texture: T) -> Option<T> {
        self.textures.insert(id.0, texture)
    }

    pub fn remove(&mut self, id: TextureId) -> Option<T> {
        self.textures.remove(&id.0)
    }

    pub fn get(&self, id: TextureId) -> Option<&T> {
        self.textures.get(&id.0)
    }

    pub fn get_mut(&mut self, id: TextureId) -> Option<&mut T> {
        self.textures.get_mut(&id.0)
    }
}
