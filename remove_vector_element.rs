fn remove_vector_element(vector: &mut Vec<i32>, index: usize) {
    // Рассчитываем длину вектора
    let vector_length = vector.len();

    // Проверка вхождения индекса в границы вектор
    if index > vector_length {
        println!("Ошибка: Индекс вышел за границу вектора.");
        return;
    }

    // Сдвигаем элементы вектора влево начиная с index
    for i in index..vector_length - 1 {
        vector[i] = vector[i + 1];
    }

    // Уменьшаем длину вектора на 1 элемент
    unsafe {
        vector.set_len(vector_length - 1);
    }
}

fn main() {
    // Задаем вектор и индекс элемента, который нужно удалить
    let mut vector = vec![1, 2, 3, 4, 5];
    let index = 3;
    
    // Рассчитываем длину вектора для вывода измененного вектора
    let vector_length = vector.len();

    // Вызываем функцию удаления элемента вектора
    remove_vector_element(&mut vector, index);

    // Если элемент находился в границах вектора, выводим измененный вектор
    if index < vector.len() {
        println!("Измененный вектор: {:?}", vector);
    }
}
