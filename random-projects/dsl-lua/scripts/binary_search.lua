function binary_search(array, value)
  local low = 1
  local high = #array

  while low <= high do
    local mid = math.floor((low + high) / 2)
    if array[mid] == value then
      return mid
    elseif array[mid] < value then
      low = mid + 1
    else
      high = mid - 1
    end
  end

  return nil
end

res = binary_search({1, 2, 3, 4, 5}, 3)
print(res)
