open System.Collections.Generic

module Input =
    type RuleMap = IDictionary<string, (uint * string) []>

    let parseLine (rule: string) =
        let parts = 
            rule.Split "contain"
            |> Array.map (fun x -> (x.Trim [|' ';','|]).Split " ")
        let bagName = parts.[0].[0] + parts.[0].[1]
        let contents =
            match parts.[1] with
            | [|"no";"other";"bags."|] -> [||]
            | _ ->
                parts.[1]
                |> Array.filter (fun x -> not (x.StartsWith "bag"))
                |> Array.chunkBySize 3
                |> Array.map (fun x -> (uint x.[0], x.[1] + x.[2]))
        (bagName, contents)

    let parseInput filename :RuleMap =
        System.IO.File.ReadAllLines filename
        |> Array.map (parseLine)
        |> dict

module Solution =
    let scanned = ref (new List<string>())

    let rec findNumContainersOf (containee:string) (rules:ref<Input.RuleMap>) = 
        if (!scanned).Contains containee then
            0u
        else
            (!scanned).Add containee
            let containers = 
                (!rules)
                |> Seq.filter (fun x -> 
                    x.Value
                    |> Array.exists (fun x -> (snd x) = containee)
                )
                |> Seq.map (fun x -> x.Key)
            let recurse x = findNumContainersOf x rules
            match Array.ofSeq containers with
            |  [||] -> 1u
            | c -> 1u + Array.sumBy recurse c

    let rec findNumContaineesOf (container:string) (rules:ref<Input.RuleMap>) =
        match (!rules).[container] with
        | [||] -> 0u
        | containees ->
            containees
            |> Array.sumBy (fun x -> fst x + fst x * findNumContaineesOf (snd x) rules)

    let part1 (rules:Input.RuleMap) =
        (findNumContainersOf "shinygold" (ref rules)) - 1u

    let part2 (rules:Input.RuleMap) =
        findNumContaineesOf "shinygold" (ref rules)

[<EntryPoint>]
let main argv =
    let input = Input.parseInput "input.txt"
    printfn "Number of bags that can eventually contain `shiny gold`: %d" (Solution.part1 input)
    printfn "Number of bags inside `shiny gold`: %d" (Solution.part2 input)
    0

