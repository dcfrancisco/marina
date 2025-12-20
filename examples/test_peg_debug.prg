// Debug version - check if disks are moving

function ShowState()
    OutStd("Peg1 (len=")
    OutStd(len1)
    OutStd("): ")
    local i := 0
    while i < len1
        OutStd(peg1[i])
        OutStd(" ")
        i := i + 1
    enddo
    OutStd("")

    OutStd("Peg2 (len=")
    OutStd(len2)
    OutStd("): ")
    i := 0
    while i < len2
        OutStd(peg2[i])
        OutStd(" ")
        i := i + 1
    enddo
    OutStd("")

    OutStd("Peg3 (len=")
    OutStd(len3)
    OutStd("): ")
    i := 0
    while i < len3
        OutStd(peg3[i])
        OutStd(" ")
        i := i + 1
    enddo
    OutStd("")
    OutStd("")
return nil

procedure Main()
    local diskCount := 3

    peg1 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
    peg2 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
    peg3 := {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}

    len1 := diskCount
    len2 := 0
    len3 := 0

    local i := 0
    while i < diskCount
        peg1[i] := diskCount - i
        i := i + 1
    enddo

    OutStd("Initial state:")
    ShowState()

    // Move disk 1 from peg1 to peg3
    len1 := len1 - 1
    local disk := peg1[len1]
    peg1[len1] := 0
    peg3[len3] := disk
    len3 := len3 + 1

    OutStd("After moving disk 1 to peg3:")
    ShowState()

return
