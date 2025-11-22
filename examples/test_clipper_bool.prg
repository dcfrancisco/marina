// Test Clipper boolean literals .T. and .F.
procedure Main()
    local isTrue := .T.
    local isFalse := .F.
    
    OutStd("Testing Clipper boolean literals:")
    OutStd("")
    
    if isTrue
        OutStd("✓ .T. is TRUE")
    endif
    
    if !isFalse
        OutStd("✓ .F. is FALSE")
    endif
    
    if .T.
        OutStd("✓ Direct .T. literal works")
    endif
    
    if !.F.
        OutStd("✓ Direct .F. literal with NOT works")
    endif
    
    // Mix with logical operators
    if .T. && .T.
        OutStd("✓ .T. && .T. is TRUE")
    endif
    
    if .T. || .F.
        OutStd("✓ .T. || .F. is TRUE")
    endif
    
    if !(.F. && .T.)
        OutStd("✓ !(.F. && .T.) is TRUE")
    endif
    
    OutStd("")
    OutStd("All Clipper boolean literal tests passed!")
return
